//! Day 12 — Exercise 2: SAXPY (c = a*x + y) with a uniform buffer. Reference solution.

use wgpu::util::DeviceExt;

const SHADER: &str = r#"
struct Scalar { a: f32, _pad: vec3<f32> }

@group(0) @binding(0) var<uniform>             params: Scalar;
@group(0) @binding(1) var<storage, read>       x:      array<f32>;
@group(0) @binding(2) var<storage, read>       y:      array<f32>;
@group(0) @binding(3) var<storage, read_write> c:      array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= arrayLength(&c)) { return; }
    c[i] = params.a * x[i] + y[i];
}
"#;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Scalar { a: f32, _pad: [f32; 3] }

pub async fn saxpy(a: f32, x: &[f32], y: &[f32]) -> Vec<f32> {
    assert_eq!(x.len(), y.len(), "x and y must have the same length");
    let n = x.len();
    if n == 0 { return Vec::new(); }
    let buf_size = (n * std::mem::size_of::<f32>()) as u64;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("device request failed");

    let scalar = Scalar { a, _pad: [0.0; 3] };
    let scalar_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("scalar"),
        contents: bytemuck::bytes_of(&scalar),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let x_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("x"),
        contents: bytemuck::cast_slice(x),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let y_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("y"),
        contents: bytemuck::cast_slice(y),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let c_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("c"),
        size: buf_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("read"),
        size: buf_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("saxpy"),
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
            wgpu::BindGroupEntry { binding: 0, resource: scalar_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: x_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: y_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: c_buf.as_entire_binding() },
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
        let wg = (n as u32).div_ceil(64);
        pass.dispatch_workgroups(wg, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&c_buf, 0, &read_buf, 0, buf_size);
    queue.submit(Some(encoder.finish()));

    let slice = read_buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map failed"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll failed");

    let data = slice.get_mapped_range();
    let out: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    read_buf.unmap();
    out
}

fn main() {
    let x: Vec<f32> = (0..16).map(|i| i as f32).collect();
    let y: Vec<f32> = (0..16).map(|i| (10 * i) as f32).collect();
    let c = pollster::block_on(saxpy(2.5, &x, &y));
    println!("c[..8] = {:?}", &c[..8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: &[f32], b: &[f32], tol: f32) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < tol)
    }

    #[test]
    fn small_batch() {
        let x = vec![1.0_f32, 2.0, 3.0, 4.0];
        let y = vec![10.0_f32, 20.0, 30.0, 40.0];
        let c = pollster::block_on(saxpy(2.5, &x, &y));
        let want = vec![12.5_f32, 25.0, 37.5, 50.0];
        assert!(approx(&c, &want, 1e-5));
    }

    #[test]
    fn longer_batch() {
        let n = 1024;
        let x: Vec<f32> = (0..n).map(|i| (i as f32) * 0.1).collect();
        let y: Vec<f32> = (0..n).map(|i| -(i as f32) * 0.05).collect();
        let a = 3.0;
        let c = pollster::block_on(saxpy(a, &x, &y));
        let want: Vec<f32> = (0..n).map(|i| a * x[i] + y[i]).collect();
        assert!(approx(&c, &want, 1e-3));
    }
}
