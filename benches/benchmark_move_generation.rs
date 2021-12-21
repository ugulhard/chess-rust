use criterion::{criterion_group, criterion_main, Criterion};
use chess_rust::chess::board::Board;

fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::new();
    c.bench_function("Move generation", |b| b.iter(|| board.legal_moves()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);