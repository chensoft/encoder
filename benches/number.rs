use criterion::*;
use encoder::number::Encode;

fn integer(c: &mut Criterion) {
    let mut buf = vec![];

    c.bench_function("i8_max", |b| b.iter(|| {
        buf.clear();
        i8::MAX.encode(&mut buf);
    }));
    c.bench_function("i8_min", |b| b.iter(|| {
        buf.clear();
        i8::MIN.encode(&mut buf);
    }));
    c.bench_function("u8_max", |b| b.iter(|| {
        buf.clear();
        u8::MAX.encode(&mut buf);
    }));
    c.bench_function("u8_min", |b| b.iter(|| {
        buf.clear();
        u8::MIN.encode(&mut buf);
    }));

    c.bench_function("i16_max", |b| b.iter(|| {
        buf.clear();
        i16::MAX.encode(&mut buf);
    }));
    c.bench_function("i16_min", |b| b.iter(|| {
        buf.clear();
        i16::MIN.encode(&mut buf);
    }));
    c.bench_function("u16_max", |b| b.iter(|| {
        buf.clear();
        u16::MAX.encode(&mut buf);
    }));
    c.bench_function("u16_min", |b| b.iter(|| {
        buf.clear();
        u16::MIN.encode(&mut buf);
    }));

    c.bench_function("i32_max", |b| b.iter(|| {
        buf.clear();
        i32::MAX.encode(&mut buf);
    }));
    c.bench_function("i32_min", |b| b.iter(|| {
        buf.clear();
        i32::MIN.encode(&mut buf);
    }));
    c.bench_function("u32_max", |b| b.iter(|| {
        buf.clear();
        u32::MAX.encode(&mut buf);
    }));
    c.bench_function("u32_min", |b| b.iter(|| {
        buf.clear();
        u32::MIN.encode(&mut buf);
    }));

    c.bench_function("i64_max", |b| b.iter(|| {
        buf.clear();
        i64::MAX.encode(&mut buf);
    }));
    c.bench_function("i64_min", |b| b.iter(|| {
        buf.clear();
        i64::MIN.encode(&mut buf);
    }));
    c.bench_function("u64_max", |b| b.iter(|| {
        buf.clear();
        u64::MAX.encode(&mut buf);
    }));
    c.bench_function("u64_min", |b| b.iter(|| {
        buf.clear();
        u64::MIN.encode(&mut buf);
    }));

    c.bench_function("i128_max", |b| b.iter(|| {
        buf.clear();
        i128::MAX.encode(&mut buf);
    }));
    c.bench_function("i128_min", |b| b.iter(|| {
        buf.clear();
        i128::MIN.encode(&mut buf);
    }));
    c.bench_function("u128_max", |b| b.iter(|| {
        buf.clear();
        u128::MAX.encode(&mut buf);
    }));
    c.bench_function("u128_min", |b| b.iter(|| {
        buf.clear();
        u128::MIN.encode(&mut buf);
    }));

    c.bench_function("isize_max", |b| b.iter(|| {
        buf.clear();
        isize::MAX.encode(&mut buf);
    }));
    c.bench_function("isize_min", |b| b.iter(|| {
        buf.clear();
        isize::MIN.encode(&mut buf);
    }));
    c.bench_function("usize_max", |b| b.iter(|| {
        buf.clear();
        usize::MAX.encode(&mut buf);
    }));
    c.bench_function("usize_min", |b| b.iter(|| {
        buf.clear();
        usize::MIN.encode(&mut buf);
    }));

    c.bench_function("true", |b| b.iter(|| {
        buf.clear();
        true.encode(&mut buf);
    }));
    c.bench_function("false", |b| b.iter(|| {
        buf.clear();
        false.encode(&mut buf);
    }));
}

fn float(c: &mut Criterion) {
    let mut buf = vec![];

    c.bench_function("f64_max", |b| b.iter(|| {
        buf.clear();
        f64::MAX.encode(&mut buf);
    }));
    c.bench_function("f64_min", |b| b.iter(|| {
        buf.clear();
        f64::MIN.encode(&mut buf);
    }));
}

criterion_group!(benches, integer, float);
criterion_main!(benches);