use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lc00018_4_sum::*;

fn benchmark_four_sum_large1(c: &mut Criterion) {
    let mut nums = vec![0];
    for i in -5..6 {
        nums.extend(vec![i;100000]);
    }
    
    let target = 7;
    c.bench_function("four_sum_v1 (3sum and 2 pointer)", |b| {
        b.iter(|| four_sum_v1(black_box(nums.clone()), black_box(target)))
    });

    c.bench_function("four_sum_v2 (HashMap)", |b| {
        b.iter(|| four_sum(black_box(nums.clone()), black_box(target)))
    });
}

fn benchmark_four_sum_large2(c: &mut Criterion) {
    let nums = (-50..50).collect::<Vec<i32>>(); // 100 numbers
    let target = 0;

    c.bench_function("four_sum_v1 (3sum and 2 pointer)", |b| {
        b.iter(|| four_sum_v1(black_box(nums.clone()), black_box(target)))
    });

    c.bench_function("four_sum_v2 (HashMap)", |b| {
        b.iter(|| four_sum(black_box(nums.clone()), black_box(target)))
    });
}

criterion_group!(benches, 
    benchmark_four_sum_large1,
    benchmark_four_sum_large2);
criterion_main!(benches);
