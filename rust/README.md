# Opteric

A tiny, simple, and fast command-line arguments reader for Rust. Maintained by [null8626](https://github.com/null8626).

Use the wrapper:
```rs
extern crate opteric;
use opteric::Opteric;
use std::env;

fn main() {
  let mut args = env::args();
  args.next().unwrap(); // skip first value - it's the path to the executable
  
  // equivalent of opteric::Iterator::from(args).collect::<Opteric>()
  println!("{}", Opteric::new(args));
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
