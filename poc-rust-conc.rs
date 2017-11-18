use std::thread;

fn main() {
  let handler = thread::spawn(|| {
    println!("fosscafe");
  });
  handler.join().unwrap();
  println!("community");
}

main();