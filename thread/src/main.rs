use std::thread;

fn main() {
  let mut children = vec![];

  for i in 0..3 {
    children.push(thread::spawn( || {
      println!("this is thread number {}", i);
    }));
  }
  for child in children {
    let _ = child.join();
  }
}