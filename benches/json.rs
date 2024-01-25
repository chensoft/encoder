use criterion::*;
use encoder::json::Encode;

fn json(c: &mut Criterion) {
    // let mut buf = vec![];
    // c.bench_function("i8_max", |b| b.iter(|| { i8::MAX.encode(&mut buf); }));
    // c.bench_function("i8_min", |b| b.iter(|| { i8::MIN.encode(&mut buf); }));
}

criterion_group!(benches, json);
criterion_main!(benches);