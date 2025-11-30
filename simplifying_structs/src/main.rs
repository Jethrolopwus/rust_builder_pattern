// simplifying struct //

struct A {
    b: B,
    c: C,


    // f1: u32,
    // f2: u32,
    // f3: u32,
}

struct B {
    f2: u32,
}

struct C {
    f1: u32,
    f3: u32,
}

fn fn1(a: &mut B) -> &u32 {
    &a.f2
}

fn fn2(a: &mut C) -> u32 {
    a.f1 + a.f3
}

fn fn3 (a: &mut A) {
    let x = fn1(&mut a.b);
    let y = fn2(&mut a.c);

    println!("{x}, {y}")

    
}

fn main() {
   let b = B { f2: 10 };
    let c = C { f1: 5, f3: 15 };
     let mut a = A { b, c };
      fn3(&mut a);
}
