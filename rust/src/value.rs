use std::fmt::{self, Display, Formatter};

pub enum Value<'a> {
  NonExistent,
  Empty,
  Exists(&'a str),
}

impl Display for Value<'_> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::NonExistent => write!(f, "<NonExistent>"),
      Self::Empty => write!(f, "<Empty>"),
      Self::Exists(ref val) => write!(f, "\"{val}\""),
    }
  }
}

impl<'a> From<&'a Option<String>> for Value<'a> {
  fn from(opt: &'a Option<String>) -> Value<'a> {
    match opt.as_ref() {
      Some(ref val) => Self::Exists(val),
      None => Self::Empty,
    }
  }
}