use std::thread;
use std::time::Duration;
use std::rc::Rc;

// fn main() {
//   let handler = thread::spawn(|| {
//     println!("fosscafe");
//   });
//   handler.join().unwrap(); //state ->
//   println!("community");
// }

//Single thread


// fn main() {
//   let mut v = vec::new()
//   //v.push(11);
//   //drop(v);
//  // v.push(3);
// }


//let x = v
//The owner can change the owning value according to mutability
//ownership model guarantees the safeness in parallel
//Cannot assign to immutable derefernce
//Mutability and Borrowing are not same
//Cannot change the value during it's borrowed

//Multithreaded programming devanshah

fn main() {
  let mut data = Rc::new(vec![1,2,3]);
  for i in 0..3{
    let d = data.clone();
    thread::spawn( move|| {
      d[i] += 1;
    });
  }
  thread::sleep(Duration::from_millis(50));
}


