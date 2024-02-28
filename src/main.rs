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
    let hex = &Twine::new_hex(&55);
    let h = t.concat(hex);
    println!("{:?}", h.to_string());
}