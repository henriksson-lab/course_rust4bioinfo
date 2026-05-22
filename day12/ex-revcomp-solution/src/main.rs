//! Day 12 — Exercise 4: reverse complement on the GPU. Reference solution.

use wgpu::util::DeviceExt;

const SHADER: &str = r#"
struct Info { n_bytes: u32, _pad: vec3<u32> }

@group(0) @binding(0) var<storage, read>       input:  array<u32>;
@group(0) @binding(1) var<uniform>             info:   Info;
@group(0) @binding(2) var<storage, read_write> output: array<u32>;

fn get_byte(i: u32) -> u32 {
    let word = input[i / 4u];
    return (word >> ((i & 3u) * 8u)) & 0xffu;
}

fn complement(b: u32) -> u32 {
    switch (b) {
        case 0x41u: { return 0x54u; }           // A -> T
        case 0x43u: { return 0x47u; }           // C -> G
        case 0x47u: { return 0x43u; }           // G -> C
        case 0x54u: { return 0x41u; }           // T -> A
        default:    { return b; }
    }
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let out_word = gid.x;
    let n_words = (info.n_bytes + 3u) / 4u;
    if (out_word >= n_words) { return; }
    var assembled: u32 = 0u;
    for (var lane: u32 = 0u; lane < 4u; lane = lane + 1u) {
        let out_byte = out_word * 4u + lane;
        if (out_byte < info.n_bytes) {
            let src = info.n_bytes - 1u - out_byte;
            let b   = complement(get_byte(src));
            assembled = assembled | (b << (lane * 8u));
        }
    }
    output[out_word] = assembled;
}
"#;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Info { n_bytes: u32, _pad: [u32; 3] }

pub fn pack_bytes(bytes: &[u8]) -> Vec<u32> {
    let n_words = bytes.len().div_ceil(4);
    let mut out = vec![0u32; n_words];
    for (i, &b) in bytes.iter().enumerate() {
        out[i / 4] |= (b as u32) << ((i % 4) * 8);
    }
    out
}

pub fn unpack_bytes(words: &[u32], n_bytes: usize) -> Vec<u8> {
    let mut out: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    out.truncate(n_bytes);
    out
}

pub fn revcomp_cpu(seq: &[u8]) -> Vec<u8> {
    seq.iter().rev().map(|&b| match b {
        b'A' => b'T', b'C' => b'G', b'G' => b'C', b'T' => b'A', _ => b,
    }).collect()
}

pub async fn revcomp_gpu(seq: &[u8]) -> Vec<u8> {
    let n_bytes = seq.len();
    if n_bytes == 0 { return Vec::new(); }
    let n_words = n_bytes.div_ceil(4);
    let packed = pack_bytes(seq);
    let out_bytes = (n_words * 4) as u64;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no GPU adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("device request failed");

    let in_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("input"),
        contents: bytemuck::cast_slice(&packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let info = Info { n_bytes: n_bytes as u32, _pad: [0; 3] };
    let info_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("info"),
        contents: bytemuck::bytes_of(&info),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let out_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("output"),
        size: out_bytes,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("read"),
        size: out_bytes,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("revcomp"),
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
        let wg = (n_words as u32).div_ceil(64);
        pass.dispatch_workgroups(wg, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, out_bytes);
    queue.submit(Some(encoder.finish()));

    let slice = read_buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map failed"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll failed");
    let data = slice.get_mapped_range();
    let words: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    read_buf.unmap();
    unpack_bytes(&words, n_bytes)
}

fn main() {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xBEEF);
    let bases = [b'A', b'C', b'G', b'T'];
    let seq: Vec<u8> = (0..32).map(|_| bases[rng.gen_range(0..4)]).collect();
    println!("input : {}", std::str::from_utf8(&seq).unwrap());
    let rc = pollster::block_on(revcomp_gpu(&seq));
    println!("output: {}", std::str::from_utf8(&rc).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};

    #[test]
    fn small_known() {
        let seq = b"ACGT".to_vec();
        let rc = pollster::block_on(revcomp_gpu(&seq));
        assert_eq!(rc, b"ACGT");        // ACGT is its own reverse complement
    }

    #[test]
    fn longer_known() {
        let seq = b"ACGTACGTACGT".to_vec();
        let rc = pollster::block_on(revcomp_gpu(&seq));
        assert_eq!(rc, revcomp_cpu(&seq));
    }

    #[test]
    fn round_trip_random() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(7);
        let bases = [b'A', b'C', b'G', b'T'];
        let seq: Vec<u8> = (0..1024).map(|_| bases[rng.gen_range(0..4)]).collect();
        let rc = pollster::block_on(revcomp_gpu(&seq));
        let back = pollster::block_on(revcomp_gpu(&rc));
        assert_eq!(seq, back);
    }

    #[test]
    fn matches_cpu_random_lengths() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(13);
        for &n in &[1usize, 4, 5, 17, 64, 65, 1024, 1025] {
            let bases = [b'A', b'C', b'G', b'T'];
            let seq: Vec<u8> = (0..n).map(|_| bases[rng.gen_range(0..4)]).collect();
            let cpu = revcomp_cpu(&seq);
            let gpu = pollster::block_on(revcomp_gpu(&seq));
            assert_eq!(cpu, gpu, "mismatch at n={n}");
        }
    }
}
