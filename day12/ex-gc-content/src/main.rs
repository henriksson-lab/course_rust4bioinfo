//! Day 12 — Exercise 3: GC content on the GPU via atomic counter. Your job: fill in the TODOs marked below.
//!
//! Compare against the day-1 / day-5 CPU versions.

use wgpu::util::DeviceExt;

const SHADER: &str = r#"
struct Info { n_bytes: u32, _pad: vec3<u32> }

@group(0) @binding(0) var<storage, read>       seq:   array<u32>;
@group(0) @binding(1) var<uniform>             info:  Info;
@group(0) @binding(2) var<storage, read_write> count: atomic<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= info.n_bytes) { return; }
    let word = seq[i / 4u];
    let shift = (i & 3u) * 8u;
    let b = (word >> shift) & 0xffu;
    if (b == 0x47u || b == 0x43u) {                // 'G' or 'C'
        atomicAdd(&count, 1u);
    }
}
"#;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Info { n_bytes: u32, _pad: [u32; 3] }

pub fn pack_bytes(bytes: &[u8]) -> Vec<u32> {
    // TODO: pack four ASCII bytes per u32, little-endian. See hint 1 in 03-gc-content.qmd.
    let _ = bytes;
    todo!("pack bytes into Vec<u32>")
}

pub fn gc_cpu(seq: &[u8]) -> u32 {
    seq.iter().filter(|&&b| b == b'G' || b == b'C').count() as u32
}

pub async fn gc_gpu(seq: &[u8]) -> u32 {
    let n_bytes = seq.len() as u32;
    if n_bytes == 0 { return 0; }
    let packed = pack_bytes(seq);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("device request failed");

    let seq_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("seq"),
        contents: bytemuck::cast_slice(&packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let info = Info { n_bytes, _pad: [0; 3] };
    let info_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("info"),
        contents: bytemuck::bytes_of(&info),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let count_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("count"),
        contents: bytemuck::bytes_of(&0u32),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });
    let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("read"),
        size: 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("gc"),
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
            wgpu::BindGroupEntry { binding: 0, resource: seq_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: info_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: count_buf.as_entire_binding() },
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
        let wg = n_bytes.div_ceil(64);
        pass.dispatch_workgroups(wg, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&count_buf, 0, &read_buf, 0, 4);
    queue.submit(Some(encoder.finish()));

    let slice = read_buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map failed"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll failed");
    let data = slice.get_mapped_range();
    let count = *bytemuck::from_bytes::<u32>(&data);
    drop(data);
    read_buf.unmap();
    count
}

fn main() {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xC0FFEE);
    let n = 1024 * 1024;
    let bases = [b'A', b'C', b'G', b'T'];
    let seq: Vec<u8> = (0..n).map(|_| bases[rng.gen_range(0..4)]).collect();
    let cpu = gc_cpu(&seq);
    let gpu = pollster::block_on(gc_gpu(&seq));
    println!("CPU GC count: {cpu}, GPU GC count: {gpu}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};

    #[test]
    fn matches_cpu_small() {
        let seq = b"ACGTACGTACGT".to_vec();
        assert_eq!(gc_cpu(&seq), 6);
        assert_eq!(pollster::block_on(gc_gpu(&seq)), 6);
    }

    #[test]
    fn all_gc() {
        let seq = b"GCGCGC".to_vec();
        assert_eq!(pollster::block_on(gc_gpu(&seq)), 6);
    }

    #[test]
    fn all_at() {
        let seq = b"AAAAAAAATTTTTTTT".to_vec();
        assert_eq!(pollster::block_on(gc_gpu(&seq)), 0);
    }

    #[test]
    fn matches_cpu_random() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let bases = [b'A', b'C', b'G', b'T'];
        let seq: Vec<u8> = (0..10_000).map(|_| bases[rng.gen_range(0..4)]).collect();
        let cpu = gc_cpu(&seq);
        let gpu = pollster::block_on(gc_gpu(&seq));
        assert_eq!(cpu, gpu);
    }
}
