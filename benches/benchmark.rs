use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lib::entity::{money::Money, payment::Payment, person::Person};
use lib::solver::solve;

fn bench_solve_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve");
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));

    let payments = vec![Payment::new(
        Money::new(1000),
        Person::new("Alice".to_string()),
        vec![
            Person::new("Bob".to_string()),
            Person::new("Charlie".to_string()),
        ],
    )];

    group.bench_function("small", |b| b.iter(|| solve(black_box(payments.clone()))));
    group.finish();
}

fn bench_solve_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve");
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));

    let payments = vec![
        Payment::new(
            Money::new(1000),
            Person::new("Alice".to_string()),
            vec![
                Person::new("Bob".to_string()),
                Person::new("Charlie".to_string()),
                Person::new("Dave".to_string()),
            ],
        ),
        Payment::new(
            Money::new(2000),
            Person::new("Bob".to_string()),
            vec![
                Person::new("Alice".to_string()),
                Person::new("Charlie".to_string()),
            ],
        ),
    ];

    group.bench_function("medium", |b| b.iter(|| solve(black_box(payments.clone()))));
    group.finish();
}

fn bench_solve_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve");
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));

    let mut payments = Vec::new();
    let names = [
        "Alice", "Bob", "Charlie", "Dave", "Eve", "Frank", "Grace", "Henry",
    ];

    // 8人で10回の支払いを生成
    for i in 0..10 {
        let payer = names[i % names.len()];
        let participants = names
            .iter()
            .filter(|&&name| name != payer)
            .take(4)
            .map(|&name| Person::new(name.to_string()))
            .collect();

        payments.push(Payment::new(
            Money::new(1000 * (i + 1) as i32),
            Person::new(payer.to_string()),
            participants,
        ));
    }

    group.bench_function("large", |b| b.iter(|| solve(black_box(payments.clone()))));
    group.finish();
}

criterion_group!(
    benches,
    bench_solve_small,
    bench_solve_medium,
    bench_solve_large
);
criterion_main!(benches);
