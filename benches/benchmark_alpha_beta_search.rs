use chess_rust::ai::ai::Ai;
use criterion::{criterion_group, criterion_main, Criterion};
use chess_rust::chess::board::Board;
use chess_rust::chess::color::Color;
use chess_rust::ai::alpha_beta_ai::AlphaBetaAi;

fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::new();
    let ai = &mut AlphaBetaAi::new(Color::White, 5);
    c.bench_function("Move generation", |b| b.iter(|| ai.find_best_move(&board)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);