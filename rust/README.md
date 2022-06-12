# Opteric

A tiny, simple, and fast command-line arguments reader for Rust.

Use the wrapper:
```rs
extern crate opteric;
use opteric::{Iterator, Opteric};
use std::env;

fn main() {
  let mut args = env::args();
  args.next().unwrap(); // skip first value - it's the path to the executable
  
  println!("{}", Iterator::from(args).collect::<Opteric>());
}
```
Or manually use the iterator:
```rs
extern crate opteric;
use opteric::Iterator;
use std::env;

fn main() {
  let mut args = env::args();
  args.next().unwrap(); // skip first value - it's the path to the executable
  
  for value in Iterator::from(args) {
    println!("{}", value);
  }
}
```