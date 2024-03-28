use iai::black_box;
use test_benchmark_iai::fibonacci;

fn iai_benchmark_short() -> u64 {
    fibonacci(black_box(10))
}

fn iai_benchmark_long() -> u64 {
    fibonacci(black_box(30))
}

iai::main!(iai_benchmark_short, iai_benchmark_long);
