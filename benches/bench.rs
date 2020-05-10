#[macro_use]
extern crate criterion;
extern crate bully;
use bully::common::bully::bully_election;
use bully::common::cli_functions::read_processes;
use bully::common::models::Storage;
use criterion::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    for i in 1..11 {
        let fname = format!("test_data/test_data_{}.txt", i);
        let content = std::fs::read_to_string(fname).expect("could not read file");
        let mut storage = Storage::new();
        read_processes(content, &mut storage);
        c.bench_function(format!("Bully bench {}", i).as_str(), |b| {
            b.iter(|| bully_election(black_box(&mut storage), false))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
