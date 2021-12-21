use criterion::{criterion_group, criterion_main, Criterion};
use chess_rust::chess::board::Board;
use chess_rust::chess::color::Color;
use chess_rust::ai::alpha_beta_ai::AlphaBetaAi;

fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::new();
    let alpha =&mut f64::MIN.clone();
    let beta =&mut f64::MAX.clone();
    let ai = &mut AlphaBetaAi::new(Color::White, 7, alpha, beta);
    c.bench_function("Move generation", |b| b.iter(|| board.legal_moves()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);