use criterion::*;
use indexmap::IndexMap;
use indexmap::IndexSet;
use encoder::json::Encode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

fn string(c: &mut Criterion) {
    let mut buf = vec![];

    c.bench_function("char", |b| b.iter(|| {
        buf.clear();
        'z'.encode(&mut buf);
    }));

    c.bench_function("str", |b| b.iter(|| {
        buf.clear();
        "Hello".encode(&mut buf);
    }));

    let str = "Hello".to_string();
    c.bench_function("string", |b| b.iter(|| {
        buf.clear();
        str.encode(&mut buf);
    }));
}

fn option(c: &mut Criterion) {
    let mut buf = vec![];
    let opt = Some(123);

    c.bench_function("option", |b| b.iter(|| {
        buf.clear();
        opt.encode(&mut buf);
    }));
}

fn array(c: &mut Criterion) {
    let array_raw = [1, 2, 3, 4, 5];
    let array_vec = vec![1, 2, 3, 4, 5];
    let array_vec_deque = VecDeque::from(array_raw.clone());

    let mut buf = vec![];

    c.bench_function("array_raw", |b| b.iter(|| {
        buf.clear();
        array_raw.encode(&mut buf);
    }));

    c.bench_function("array_vec", |b| b.iter(|| {
        buf.clear();
        array_vec.encode(&mut buf);
    }));

    c.bench_function("array_vec_deque", |b| b.iter(|| {
        buf.clear();
        array_vec_deque.encode(&mut buf);
    }));

    let set_hash = HashSet::from(array_raw.clone());
    let set_btree = BTreeSet::from(array_raw.clone());
    let set_index = IndexSet::from(array_raw.clone());

    c.bench_function("set_hash", |b| b.iter(|| {
        buf.clear();
        set_hash.encode(&mut buf);
    }));

    c.bench_function("set_btree", |b| b.iter(|| {
        buf.clear();
        set_btree.encode(&mut buf);
    }));

    c.bench_function("set_index", |b| b.iter(|| {
        buf.clear();
        set_index.encode(&mut buf);
    }));
}

fn object(c: &mut Criterion) {
    let data = [("key1", 111), ("key2", 222), ("key3", 333)];
    let object_hash = HashMap::from(data.clone());
    let object_btree = BTreeMap::from(data.clone());
    let object_index = IndexMap::from(data.clone());

    let mut buf = vec![];

    c.bench_function("object_hash", |b| b.iter(|| {
        buf.clear();
        object_hash.encode(&mut buf);
    }));

    c.bench_function("object_btree", |b| b.iter(|| {
        buf.clear();
        object_btree.encode(&mut buf);
    }));

    c.bench_function("object_index", |b| b.iter(|| {
        buf.clear();
        object_index.encode(&mut buf);
    }));
}

criterion_group!(benches, string, option, array, object);
criterion_main!(benches);