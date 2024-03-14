use criterion::{black_box, criterion_group, criterion_main, Criterion};
use twine::Twine;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g_str_str_short = c.benchmark_group("short str concat");
    let str_str_short = ("foo", "bar");
    g_str_str_short.bench_with_input("Twine: concat ", &str_str_short, |bench, &(a, b)| {
        bench.iter(|| {
            let t1 = &Twine::from(a);
            let t2 = &Twine::from(b);
            let r = t1 + t2;
            let _ = black_box(r);
        })
    });
    g_str_str_short.bench_with_input(
        "Twine: concat + to_string_preallocating",
        &str_str_short,
        |bench, &(a, b)| {
            bench.iter(|| {
                let t1 = &Twine::from(a);
                let t2 = &Twine::from(b);
                let a = t1 + t2;
                let r = a.to_string_preallocating();
                let _ = black_box(r);
            })
        },
    );
    g_str_str_short.bench_with_input(
        "Twine: concat + to_string",
        &str_str_short,
        |bench, &(a, b)| {
            bench.iter(|| {
                let t1 = &Twine::from(a);
                let t2 = &Twine::from(b);
                let a = t1 + t2;
                let r = a.to_string();
                let _ = black_box(r);
            })
        },
    );
    g_str_str_short.bench_with_input("String: concat", &str_str_short, |bench, &(a, b)| {
        bench.iter(|| {
            let r = String::from(a) + b;
            let _ = black_box(r);
        })
    });
    g_str_str_short.bench_with_input("format_args: concat", &str_str_short, |bench, &(a, b)| {
        bench.iter(|| {
            let _ = black_box(format_args!("{}{}", a, b));
        })
    });
    g_str_str_short.bench_with_input(
        "format_args: concat + to_string",
        &str_str_short,
        |bench, &(a, b)| {
            bench.iter(|| {
                let r = format_args!("{}{}", a, b).to_string();
                let _ = black_box(r);
            })
        },
    );
    g_str_str_short.finish();

    let mut g_str_str_long = c.benchmark_group("long str concat");
    let str_str_long = (
        "1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890",
        "1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890",
    );
    g_str_str_long.bench_with_input("Twine: concat ", &str_str_long, |bench, &(a, b)| {
        bench.iter(|| {
            let t1 = &Twine::from(a);
            let t2 = &Twine::from(b);
            let r = t1 + t2;
            let _ = black_box(r);
        })
    });
    g_str_str_long.bench_with_input(
        "Twine: concat + to_string_preallocating",
        &str_str_long,
        |bench, &(a, b)| {
            bench.iter(|| {
                let t1 = &Twine::from(a);
                let t2 = &Twine::from(b);
                let a = t1 + t2;
                let r = a.to_string_preallocating();
                let _ = black_box(r);
            })
        },
    );
    g_str_str_long.bench_with_input(
        "Twine: concat + to_string",
        &str_str_long,
        |bench, &(a, b)| {
            bench.iter(|| {
                let t1 = &Twine::from(a);
                let t2 = &Twine::from(b);
                let a = t1 + t2;
                let r = a.to_string();
                let _ = black_box(r);
            })
        },
    );
    g_str_str_long.bench_with_input("String: concat", &str_str_long, |bench, &(a, b)| {
        bench.iter(|| {
            let r = String::from(a) + b;
            let _ = black_box(r);
        })
    });
    g_str_str_long.bench_with_input("format_args: concat", &str_str_long, |bench, &(a, b)| {
        bench.iter(|| {
            let _ = black_box(format_args!("{}{}", a, b));
        })
    });
    g_str_str_long.bench_with_input(
        "format_args: concat + to_string",
        &str_str_long,
        |bench, &(a, b)| {
            bench.iter(|| {
                let r = format_args!("{}{}", a, b).to_string();
                let _ = black_box(r);
            })
        },
    );
    g_str_str_long.finish();

    let mut g_str_u32 = c.benchmark_group("str concat u32");
    let str_u32 = ("identifier-", &4321u32);
    g_str_u32.bench_with_input("Twine: concat ", &str_u32, |bench, &(a, b)| {
        bench.iter(|| {
            let t1 = &Twine::from(a);
            let t2 = &Twine::from(b);
            let r = t1 + t2;
            let _ = black_box(r);
        })
    });
    g_str_u32.bench_with_input(
        "Twine: concat + to_string_preallocating",
        &str_u32,
        |bench, &(a, b)| {
            bench.iter(|| {
                let t1 = &Twine::from(a);
                let t2 = &Twine::from(b);
                let a = t1 + t2;
                let r = a.to_string_preallocating();
                let _ = black_box(r);
            })
        },
    );
    g_str_u32.bench_with_input("Twine: concat + to_string", &str_u32, |bench, &(a, b)| {
        bench.iter(|| {
            let t1 = &Twine::from(a);
            let t2 = &Twine::from(b);
            let a = t1 + t2;
            let r = a.to_string();
            let _ = black_box(r);
        })
    });
    g_str_u32.bench_with_input("String: format", &str_u32, |bench, &(a, b)| {
        bench.iter(|| {
            let r = format!("{}{}", a, b);
            let _ = black_box(r);
        })
    });
    g_str_u32.bench_with_input("format_args: concat", &str_u32, |bench, &(a, b)| {
        bench.iter(|| {
            let _ = black_box(format_args!("{}{}", a, b));
        })
    });
    g_str_u32.bench_with_input(
        "format_args: concat + to_string",
        &str_u32,
        |bench, &(a, b)| {
            bench.iter(|| {
                let r = format_args!("{}{}", a, b).to_string();
                let _ = black_box(r);
            })
        },
    );
    g_str_u32.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
