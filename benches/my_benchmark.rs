use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use charbench::{ is_reserved_char_1, is_reserved_char_6 };

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is_reserved_char_1('a')",
                     |b| b.iter(|| is_reserved_char_1(black_box('a'))));
    c.bench_function("is_reserved_char_1('b')",
                     |b| b.iter(|| is_reserved_char_1(black_box('b'))));
    c.bench_function("is_reserved_char_1('o')",
                     |b| b.iter(|| is_reserved_char_1(black_box('o'))));
    c.bench_function("is_reserved_char_1('9')",
                     |b| b.iter(|| is_reserved_char_1(black_box('9'))));

    c.bench_function("is_reserved_char_6('a')",
                     |b| b.iter(|| is_reserved_char_6(black_box('a'))));
    c.bench_function("is_reserved_char_6('b')",
                     |b| b.iter(|| is_reserved_char_6(black_box('b'))));
    c.bench_function("is_reserved_char_6('o')",
                     |b| b.iter(|| is_reserved_char_6(black_box('o'))));
    c.bench_function("is_reserved_char_6('9')",
                     |b| b.iter(|| is_reserved_char_6(black_box('9'))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
