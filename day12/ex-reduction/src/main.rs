//! Day 12 — Exercise 5: parallel sum reduction via workgroup shared memory. Your job: fill in the TODOs marked below.

use wgpu::util::DeviceExt;

const SHADER: &str = r#"
struct Info { n: u32, _pad: vec3<u32> }

@group(0) @binding(0) var<storage, read>       input:    array<f32>;
@group(0) @binding(1) var<uniform>             info:     Info;
@group(0) @binding(2) var<storage, read_write> partials: array<f32>;

var<workgroup> tile: array<f32, 64>;

@compute @workgroup_size(64)
fn main(
    @builtin(global_invocation_id) gid: vec3<u32>,
    @builtin(local_invocation_id)  lid: vec3<u32>,
    @builtin(workgroup_id)         wid: vec3<u32>,
) {
    let i = gid.x;
    tile[lid.x] = select(0.0, input[i], i < info.n);
    workgroupBarrier();

    var stride: u32 = 32u;
    loop {
        if (stride == 0u) { break; }
        if (lid.x < stride) {
            tile[lid.x] = tile[lid.x] + tile[lid.x + stride];
        }
        workgroupBarrier();
        stride = stride / 2u;
    }

    if (lid.x == 0u) { partials[wid.x] = tile[0]; }
}
"#;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Info { n: u32, _pad: [u32; 3] }

pub async fn sum_gpu(x: &[f32]) -> f32 {
    let n = x.len();
    if n == 0 { return 0.0; }

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("device request failed");

    let workgroups = (n as u32).div_ceil(64);
    let partials_bytes = (workgroups as u64) * 4;

    let in_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("input"),
        contents: bytemuck::cast_slice(x),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let info = Info { n: n as u32, _pad: [0; 3] };
    let info_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("info"),
        contents: bytemuck::bytes_of(&info),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let partials_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("partials"),
        size: partials_bytes.max(4),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("read"),
        size: partials_bytes.max(4),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("reduce"),
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
            wgpu::BindGroupEntry { binding: 0, resource: in_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: info_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: partials_buf.as_entire_binding() },
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
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&partials_buf, 0, &read_buf, 0, partials_bytes.max(4));
    queue.submit(Some(encoder.finish()));

    let slice = read_buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map failed"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll failed");
    let data = slice.get_mapped_range();
    let partials: Vec<f32> = bytemuck::cast_slice(&data[..(partials_bytes as usize)]).to_vec();
    drop(data);
    read_buf.unmap();
    // TODO: sum the per-workgroup partials on the host. See hint 2 in 05-reduction.qmd.
    let _ = partials;
    todo!("return partials.iter().sum()")
}

fn main() {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEAD);
    let n = 1_000_000;
    let x: Vec<f32> = (0..n).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let cpu: f32 = x.iter().sum();
    let gpu = pollster::block_on(sum_gpu(&x));
    println!("CPU: {cpu:.6}, GPU: {gpu:.6}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};

    fn close(a: f32, b: f32, rel: f32) -> bool {
        let scale = a.abs().max(b.abs()).max(1.0);
        (a - b).abs() <= rel * scale
    }

    #[test]
    fn empty_sum() {
        assert_eq!(pollster::block_on(sum_gpu(&[])), 0.0);
    }

    #[test]
    fn small_known() {
        let x: Vec<f32> = (1..=10).map(|i| i as f32).collect();
        let got = pollster::block_on(sum_gpu(&x));
        assert!((got - 55.0).abs() < 1e-5);
    }

    #[test]
    fn matches_cpu_medium() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);
        let x: Vec<f32> = (0..10_000).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let cpu: f32 = x.iter().sum();
        let gpu = pollster::block_on(sum_gpu(&x));
        assert!(close(cpu, gpu, 1e-4));
    }

    #[test]
    fn matches_cpu_non_multiple_of_64() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(2);
        let x: Vec<f32> = (0..1000).map(|_| rng.gen_range(0.0..10.0)).collect();
        let cpu: f32 = x.iter().sum();
        let gpu = pollster::block_on(sum_gpu(&x));
        assert!(close(cpu, gpu, 1e-4));
    }
}
