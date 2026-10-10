use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use mschief::lexer::Lexer;

fn lexer_bench(c: &mut Criterion) {
    let samples = [
        (
            "Sample 1",
            include_str!("../third/FinalScoreMC/generators/structure.library/dungeon/generator.ms"),
        ),
        (
            "Sample 2",
            include_str!("../third/FinalScoreMC/games/shard/objectives.library/scavengerhunt.ms"),
        ),
        (
            "Sample 3",
            include_str!(
                "../third/FinalScoreMC/generators/structure.library/scaled_level/generator.ms"
            ),
        ),
    ];

    let mut group = c.benchmark_group("Lexer");

    for (name, source) in samples.iter() {
        group.throughput(Throughput::Bytes(source.len() as u64));

        group.bench_with_input(BenchmarkId::from_parameter(name), &source, |b, &src| {
            b.iter(|| {
                let lexer = Lexer::new(black_box(src));
                lexer.for_each(|t| {
                    let _ = black_box(t);
                });
            })
        });
    }
}

criterion_group!(benches, lexer_bench);
criterion_main!(benches);
