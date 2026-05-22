//! Day 12 — Exercise 6: pairwise distance matrix on the GPU (2-D dispatch). Your job: fill in the TODOs marked below.

use wgpu::util::DeviceExt;

const SHADER: &str = r#"
struct Info { n: u32, d: u32, _pad: vec2<u32> }

@group(0) @binding(0) var<storage, read>       x:    array<f32>;
@group(0) @binding(1) var<uniform>             info: Info;
@group(0) @binding(2) var<storage, read_write> out:  array<f32>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    let j = gid.y;
    if (i >= info.n || j >= info.n) { return; }
    let n = info.n;
    let d = info.d;
    var sum: f32 = 0.0;
    for (var k: u32 = 0u; k < d; k = k + 1u) {
        let a = x[i * d + k];
        let b = x[j * d + k];
        let diff = a - b;
        sum = sum + diff * diff;
    }
    out[i * n + j] = sqrt(sum);
}
"#;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Info { n: u32, d: u32, _pad: [u32; 2] }

pub fn pairwise_l2_cpu(x: &[f32], n: usize, d: usize) -> Vec<f32> {
    assert_eq!(x.len(), n * d);
    let mut out = vec![0.0_f32; n * n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..d {
                let diff = x[i * d + k] - x[j * d + k];
                s += diff * diff;
            }
            out[i * n + j] = s.sqrt();
        }
    }
    out
}

pub async fn pairwise_l2_gpu(x: &[f32], n: usize, d: usize) -> Vec<f32> {
    assert_eq!(x.len(), n * d);
    if n == 0 { return Vec::new(); }
    let out_bytes = (n * n * std::mem::size_of::<f32>()) as u64;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("device request failed");

    let x_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("x"),
        contents: bytemuck::cast_slice(x),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let info = Info { n: n as u32, d: d as u32, _pad: [0; 2] };
    let info_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("info"),
        contents: bytemuck::bytes_of(&info),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let out_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("out"),
        size: out_bytes.max(4),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("read"),
        size: out_bytes.max(4),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("dist"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("pipeline"),
        layout: None,
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    let bgl = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg"),
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: x_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: info_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: out_buf.as_entire_binding() },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        // TODO: dispatch a 2-D grid of workgroups (workgroup_size = 8x8). See hint 1 in 06-pairwise-distance.qmd.
        let _ = n;
        let wg: u32 = todo!("ceiling-divide n by 8");
        pass.dispatch_workgroups(wg, wg, 1);
    }
    encoder.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, out_bytes.max(4));
    queue.submit(Some(encoder.finish()));

    let slice = read_buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map failed"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll failed");
    let data = slice.get_mapped_range();
    let out: Vec<f32> = bytemuck::cast_slice(&data[..(out_bytes as usize)]).to_vec();
    drop(data);
    read_buf.unmap();
    out
}

fn main() {
    let pts: Vec<f32> = vec![
        0.0, 0.0,
        3.0, 4.0,
        6.0, 0.0,
        3.0, -4.0,
    ];
    let n = 4;
    let d = 2;
    let dist = pollster::block_on(pairwise_l2_gpu(&pts, n, d));
    for i in 0..n { for j in 0..n {
        print!("{:6.2}  ", dist[i * n + j]);
    } println!(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32, tol: f32) -> bool { (a - b).abs() < tol }

    fn classic_points() -> (Vec<f32>, usize, usize) {
        let pts: Vec<f32> = vec![
            0.0, 0.0,
            3.0, 4.0,
            6.0, 0.0,
            3.0, -4.0,
        ];
        (pts, 4, 2)
    }

    #[test]
    fn shape_and_diagonal() {
        let (pts, n, d) = classic_points();
        let dist = pollster::block_on(pairwise_l2_gpu(&pts, n, d));
        assert_eq!(dist.len(), n * n);
        for i in 0..n { assert!(close(dist[i * n + i], 0.0, 1e-4)); }
    }

    #[test]
    fn symmetry() {
        let (pts, n, d) = classic_points();
        let dist = pollster::block_on(pairwise_l2_gpu(&pts, n, d));
        for i in 0..n { for j in 0..n {
            assert!(close(dist[i * n + j], dist[j * n + i], 1e-4));
        }}
    }

    #[test]
    fn classic_distances() {
        let (pts, n, d) = classic_points();
        let dist = pollster::block_on(pairwise_l2_gpu(&pts, n, d));
        assert!(close(dist[0 * n + 1], 5.0, 1e-4));
        assert!(close(dist[0 * n + 2], 6.0, 1e-4));
        assert!(close(dist[0 * n + 3], 5.0, 1e-4));
        assert!(close(dist[1 * n + 2], 5.0, 1e-4));
        assert!(close(dist[1 * n + 3], 8.0, 1e-4));
    }

    #[test]
    fn matches_cpu_random() {
        let n = 64;
        let d = 16;
        let pts: Vec<f32> = (0..n * d).map(|i| (((i * 37) % 100) as f32) * 0.1).collect();
        let cpu = pairwise_l2_cpu(&pts, n, d);
        let gpu = pollster::block_on(pairwise_l2_gpu(&pts, n, d));
        assert_eq!(cpu.len(), gpu.len());
        for (i, (c, g)) in cpu.iter().zip(&gpu).enumerate() {
            assert!(close(*c, *g, 1e-3), "mismatch at {i}: cpu={c} gpu={g}");
        }
    }
}
