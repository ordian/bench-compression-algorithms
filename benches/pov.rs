use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use parity_scale_codec::{Encode, Decode};
use std::io::{Read as _, Write as _};

criterion_main!(benches);
criterion_group!(
    benches,
    bench_cmp_compression,
);

static KUSAMA: &[u8] = include_bytes!("wasm/kusama_runtime.compact.wasm");

#[derive(Encode, Decode)]
struct RuntimeUpgrade {
    wasm: Vec<u8>,
}

impl From<&[u8]> for RuntimeUpgrade {
    fn from(wasm: &[u8]) -> RuntimeUpgrade {
        RuntimeUpgrade { wasm: wasm.to_vec() }
    }
}

fn bench_cmp_compression(c: &mut Criterion) {
    let mut group_encode = c.benchmark_group("encode");

    let scale_encoded = RuntimeUpgrade::from(KUSAMA).encode();

    group_encode.throughput(Throughput::Bytes(scale_encoded.len() as u64));
    group_encode.bench_function("gzip(4)",|b| b.iter(|| gzip_encode(&scale_encoded, 4)));
    group_encode.bench_function("gzip(6)",|b| b.iter(|| gzip_encode(&scale_encoded, 6)));
    group_encode.bench_function("lz4(1)", |b| b.iter(|| lz4_encode(&scale_encoded, 1)));
    group_encode.bench_function("lz4(4)", |b| b.iter(|| lz4_encode(&scale_encoded, 4)));
    group_encode.bench_function("zstd(3)", |b| b.iter(|| zstd_encode(&scale_encoded, 3)));
    group_encode.bench_function("zstd(10)", |b| b.iter(|| zstd_encode(&scale_encoded, 10)));

    let gzip_4_encoded = gzip_encode(&scale_encoded, 4);
    let gzip_6_encoded = gzip_encode(&scale_encoded, 6);
    let lz4_1_encoded = lz4_encode(&scale_encoded, 1);
    let lz4_4_encoded = lz4_encode(&scale_encoded, 4);
    let zstd_3_encoded = zstd_encode(&scale_encoded, 3);
    let zstd_10_encoded = zstd_encode(&scale_encoded, 10);
    let ruzstd_3_decoded = ruzstd_decode(&zstd_3_encoded);
    assert_eq!(&ruzstd_3_decoded, &scale_encoded);

    eprintln!("");
    eprintln!("Compression ratios:");
    eprintln!("  gzip(4): {}", scale_encoded.len() as f32 / gzip_4_encoded.len() as f32);
    eprintln!("  gzip(6): {}", scale_encoded.len() as f32 / gzip_6_encoded.len() as f32);
    eprintln!("  lz4(1): {}", scale_encoded.len() as f32 / lz4_1_encoded.len() as f32);
    eprintln!("  lz4(4): {}", scale_encoded.len() as f32 / lz4_4_encoded.len() as f32);
    eprintln!("  zstd(3): {}", scale_encoded.len() as f32 / zstd_3_encoded.len() as f32);
    eprintln!("  zstd(10): {}", scale_encoded.len() as f32 / zstd_10_encoded.len() as f32);

    group_encode.finish();

    let mut group_decode = c.benchmark_group("decode");
    group_decode.bench_function("gzip(4)",|b| b.iter(|| gzip_decode(&gzip_4_encoded)));
    group_decode.bench_function("gzip(6)",|b| b.iter(|| gzip_decode(&gzip_6_encoded)));
    group_decode.bench_function("lz4(1)", |b| b.iter(|| lz4_decode(&lz4_1_encoded)));
    group_decode.bench_function("lz4(4)", |b| b.iter(|| lz4_decode(&lz4_4_encoded)));
    group_decode.bench_function("zstd(3)", |b| b.iter(|| zstd_decode(&zstd_3_encoded)));
    group_decode.bench_function("zstd(10)", |b| b.iter(|| zstd_decode(&zstd_10_encoded)));
    group_decode.bench_function("ruzstd(3)", |b| b.iter(|| ruzstd_decode(&zstd_3_encoded)));
    group_decode.finish();
}

fn gzip_encode(data: &[u8], level: u32) -> Vec<u8> {
    use flate2::{write::GzEncoder, Compression};

    let mut encoder = GzEncoder::new(Vec::with_capacity(1024), Compression::new(level));
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn lz4_encode(data: &[u8], level: u32) -> Vec<u8> {
    let mut encoder = lz4::EncoderBuilder::new()
        .level(level)
        .build(Vec::with_capacity(1024)).unwrap();

    encoder.write_all(data).unwrap();
    encoder.finish().0
}

fn zstd_encode(data: &[u8], level: i32) -> Vec<u8> {
    use zstd::stream::Encoder;

    let mut encoder = Encoder::new(Vec::with_capacity(1024), level).unwrap();
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}


fn gzip_decode(data: &[u8]) -> Vec<u8> {
    use flate2::write::GzDecoder;

    let mut decoder = GzDecoder::new(Vec::with_capacity(1024));
    decoder.write_all(data).unwrap();
    decoder.finish().unwrap()
}

fn lz4_decode(data: &[u8]) -> Vec<u8> {
    let mut decoder = lz4::Decoder::new(data).unwrap();
    let mut buffer = Vec::with_capacity(1024);
    decoder.read_to_end(&mut buffer).unwrap();
    buffer
}

fn zstd_decode(data: &[u8]) -> Vec<u8> {
    use zstd::stream::read::Decoder;

    let mut decoder = Decoder::new(data).unwrap();
    let mut buffer = Vec::with_capacity(1024);
    decoder.read_to_end(&mut buffer).unwrap();
    buffer
}

fn ruzstd_decode(mut data: &[u8]) -> Vec<u8> {
    use ruzstd::streaming_decoder::StreamingDecoder;

    let mut decoder = StreamingDecoder::new(&mut data).unwrap();
    let mut buffer = Vec::with_capacity(1024);
    decoder.read_to_end(&mut buffer).unwrap();
    buffer
}
