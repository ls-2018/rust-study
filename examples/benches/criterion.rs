use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    id: u32,
    name: String,
    values: Vec<i32>,
}

fn serialize_example(data: &Example) -> String {
    serde_json::to_string(data).unwrap()
}

fn deserialize_example(json_str: &str) -> Example {
    serde_json::from_str(json_str).unwrap()
}

fn benchmark(c: &mut Criterion) {
    let example = Example {
        id: 1,
        name: "Example".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };

    let json_str = serialize_example(&example);

    c.bench_function("serialize", |b| {
        b.iter(|| serialize_example(black_box(&example)))
    });

    c.bench_function("deserialize", |b| {
        b.iter(|| deserialize_example(black_box(&json_str)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
