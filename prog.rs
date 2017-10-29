struct Foo {
  f: Box<int>,
}

fn main() {
  let mut a = Foo {f: box 0};
  let x = &mut a;
    println!("{}", a.f);
}

//Mutable borrow
//
//error : cannot borrow
//
//&mut: T single instance