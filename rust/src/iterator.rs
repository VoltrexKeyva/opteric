use std::fmt::{self, Display, Formatter};
use crate::Value;
use std::iter;

pub struct Iterator<I> {
  found_flags: bool,
  previous_flag: Option<String>,
  args: I,
}

impl<I: iter::Iterator<Item = String>> Iterator<I> {
  fn get_flag(s: &str) -> Option<String> {
    let mut c = s.chars();
  
    if c.next()? != '-' {
      None
    } else if s.len() > 2 && c.next().unwrap() == '-' {
      Some(s[2..].to_string())
    } else {
      Some(s[1..].to_string())
    }
  }
}

impl<I: iter::Iterator<Item = String>> From<I> for Iterator<I> {
  #[inline]
  fn from(args: I) -> Self {
    Self {
      found_flags: false,
      previous_flag: None,
      args,
    }
  }
}

pub enum IteratorOutput {
  NonOption(String),
  Option(String, Option<String>),
}

impl Display for IteratorOutput {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::NonOption(ref val) => write!(f, "NonOption(\"{val}\")"),
      Self::Option(ref key, ref val) => write!(f, "Option({} => {})", key, Value::from(val)),
    }
  }
}

impl<I: iter::Iterator<Item = String>> iter::Iterator for Iterator<I> {
  type Item = IteratorOutput;
  
  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.args.next() {
        Some(args) => {
          let flag = Self::get_flag(&args);
              
          if !self.found_flags {
            if flag.is_some() {
              self.found_flags = true;
            } else {
              return Some(IteratorOutput::NonOption(args));
            }
          }
          
          if let Some(f) = flag {
            if self.previous_flag.is_some() {
              return Some(IteratorOutput::Option(self.previous_flag.take().unwrap(), None));
            } else {
              self.previous_flag.replace(f);
            }
          } else {
            if self.previous_flag.is_some() {
              return Some(IteratorOutput::Option(self.previous_flag.take().unwrap(), Some(args)));
            } else {
              return Some(IteratorOutput::NonOption(args));
            }
          }
        },
          
        None => {
          if self.previous_flag.is_some() {
            return Some(IteratorOutput::Option(self.previous_flag.take().unwrap(), None));
          } else {
            return None;
          }
        }
      }
    }  
  }
}
