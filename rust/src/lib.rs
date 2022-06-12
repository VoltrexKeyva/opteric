use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;
use std::ops::Index;
use std::iter;

mod value;
pub use value::Value;

mod iterator;
pub use iterator::{Iterator, IteratorOutput};
pub type Options = HashMap<String, Option<String>>;

pub struct Opteric {
  inner_non_options: Vec<String>,
  inner_map: Options,
}

impl<'a> Opteric {
  pub fn get(&'a self, key: &str) -> Value<'a> {
    if let Some(ref exists) = self.inner_map.get(key) {
      if let Some(ref value) = exists {
        Value::Exists(value)
      } else {
        Value::Empty
      }
    } else {
      Value::NonExistent
    }
  }
  
  #[inline]
  pub fn non_options(&'a self) -> &'a [String] {
    &self.inner_non_options
  }
}

impl iter::FromIterator<IteratorOutput> for Opteric {
  fn from_iter<I: iter::IntoIterator<Item = IteratorOutput>>(it: I) -> Self {
    let mut inner_map = Options::new();
    let mut inner_non_options: Vec<String> = Vec::new();
    
    for val in it {
      match val {
        IteratorOutput::NonOption(s) => inner_non_options.push(s),
        IteratorOutput::Option(k, v) => { inner_map.insert(k, v); },
      }
    }
    
    Self {
      inner_non_options,
      inner_map,
    }
  }
}

impl AsRef<Options> for Opteric {
  #[inline]
  fn as_ref<'a>(&'a self) -> &'a Options {
    &self.inner_map
  }
}

impl Index<&'static str> for Opteric {
  type Output = String;
  
  fn index(&self, index: &'static str) -> &String {
    self.inner_map.get(index)
      .expect("Value does not exist.")
      .as_ref()
      .expect("Value does not have a value.")
  }
}

impl Display for Opteric {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "[")?;
    
    let mut i = 0;
    
    for (key, value) in self.inner_map.iter() {
      write!(f, "({} => {})", key, Value::from(value))?;
    
      i += 1;
      
      if i == self.inner_map.len() {
        write!(f, "]")?;
      } else {
        write!(f, ", ")?;
      }
    }
    
    Ok(())
  }
}