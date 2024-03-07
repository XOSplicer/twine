use criterion::{black_box, criterion_group, criterion_main, Criterion};
use twine::Twine;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Twine: str concat short", |b| {
        b.iter(|| {
            let t1 = &Twine::from("foo");
            let t2 = &Twine::from("bar");
            let r = t1 + t2;
            let _ = black_box(r);
        })
    });
    c.bench_function("Twine: str concat short + to_string_preallocating", |b| {
        b.iter(|| {
            let t1 = &Twine::from("foo");
            let t2 = &Twine::from("bar");
            let a = t1 + t2;
            let r = a.to_string_preallocating();
            let _ = black_box(r);
        })
    });
    c.bench_function("Twine: str concat short + to_string", |b| {
        b.iter(|| {
            let t1 = &Twine::from("foo");
            let t2 = &Twine::from("bar");
            let a = t1 + t2;
            let r = a.to_string();
            let _ = black_box(r);
        })
    });
    c.bench_function("String: str concat short", |b| {
        b.iter(|| {
            let r = String::from("foo") + "bar";
            let _ = black_box(r);
        })
    });
    c.bench_function("Twine: str concat long", |b| b.iter(|| {
        let t1 = &Twine::from("1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890");
        let t2 = &Twine::from("1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890");
        let r = t1 + t2;
        let _ = black_box(r);
    }));
    c.bench_function("Twine: str concat long + to_string_preallocating", |b| b.iter(|| {
        let t1 = &Twine::from("1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890");
        let t2 = &Twine::from("1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890");
        let a = t1 + t2;
        let r = a.to_string_preallocating();
        let _ = black_box(r);
    }));
    c.bench_function("Twine: str concat long + to_string", |b| b.iter(|| {
        let t1 = &Twine::from("1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890");
        let t2 = &Twine::from("1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890");
        let a = t1 + t2;
        let r = a.to_string();
        let _ = black_box(r);
    }));
    c.bench_function("String: str concat long", |b| b.iter(|| {
        let s1 = String::from("1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890");
        let s2 = "1234567890123456789012345789012345678901234567890123457890123457890123456789012345678901234567890123456789012345678901234567890";
        let r = s1 + &s2;
        let _ = black_box(r);
    }));
    c.bench_function("Twine: str concat u32", |b| {
        b.iter(|| {
            let t1 = &Twine::from("identifier-");
            let t2 = &Twine::from(&4321u32);
            let r = t1 + t2;
            let _ = black_box(r);
        })
    });
    c.bench_function("Twine: str concat u32 + to_string", |b| {
        b.iter(|| {
            let t1 = &Twine::from("identifier-");
            let t2 = &Twine::from(&4321u32);
            let a = t1 + t2;
            let r = a.to_string();
            let _ = black_box(r);
        })
    });
    c.bench_function("Twine: str concat u32 + to_string_preallocating", |b| {
        b.iter(|| {
            let t1 = &Twine::from("identifier-");
            let t2 = &Twine::from(&4321u32);
            let a = t1 + t2;
            let r = a.to_string_preallocating();
            let _ = black_box(r);
        })
    });
    c.bench_function("String: format str concat u32", |b| {
        b.iter(|| {
            let s1 = String::from("identifier-");
            let u = 4321u32;
            let r = format!("{}{}", &s1, &u);
            let _ = black_box(r);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
