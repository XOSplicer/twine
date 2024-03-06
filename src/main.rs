use twine::Twine;

fn main() {

    println!("sizeof(Twine)={}", std::mem::size_of::<Twine<'static>>());
    println!("sizeof(usize)={}", std::mem::size_of::<usize>());
    println!("sizeof(&str)={}", std::mem::size_of::<&str>());
    let bar = Twine::from("bar");
    println!("{:?}", &bar);
    let s = Twine::from(" ");
    println!("{:?}", &s);
    let foo = Twine::from("foo");
    println!("{:?}", &foo);
    let r = foo.concat(&s);
    println!("{:?}", &r);
    let t = r.concat(&bar);
    println!("{:?}", &t);
    let string = t.to_string();
    println!("{:?}", &string);
    let hex = &Twine::hex(&55);
    let h = t.concat(hex);
    println!("{:?}", h.to_string());
    let h2 = h.concat(&h);
    println!("{:?}", h2.to_string());

    let bump = bumpalo::Bump::new();
    dbg!(bump.allocated_bytes());
    let base = bump.alloc_str("bumpalloc-");
    dbg!(bump.allocated_bytes());
    let t = &*bump.alloc(Twine::from(&*base));
    dbg!(bump.allocated_bytes());
    let t1 = t + &*bump.alloc(Twine::hex(&1));
    dbg!(bump.allocated_bytes());
    let mut s1 = bumpalo::collections::String::with_capacity_in(t1.estimated_capacity().next_power_of_two(), &bump);
    dbg!(bump.allocated_bytes());
    let _ = t1.write_to(&mut s1);
    dbg!(bump.allocated_bytes());
    println!("{:?}", &s1);
    dbg!(s1.capacity());
    dbg!(bump.allocated_bytes());
}