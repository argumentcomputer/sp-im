#![allow(clippy::unit_arg)]

use std::{
  collections::BTreeSet,
  fmt::{
    Debug,
    Error,
    Formatter,
    Write,
  },
};

use crate::OrdSet;
use rand::Rng;

use quickcheck::{
  Arbitrary,
  Gen,
};

#[derive(Debug, Clone)]
enum Action<A> {
  Insert(A),
  Remove(A),
}

#[derive(Clone)]
struct Actions<A>(Vec<Action<A>>)
where A: Ord + Clone;

impl<A: Arbitrary + Ord> Arbitrary for Action<A> {
  fn arbitrary(g: &mut Gen) -> Self {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=1) {
      0 => Action::Insert(A::arbitrary(g)),
      _ => Action::Remove(A::arbitrary(g)),
    }
  }
}

impl<A: Arbitrary + Ord> Arbitrary for Actions<A> {
  fn arbitrary(g: &mut Gen) -> Self { Actions(Vec::<Action<A>>::arbitrary(g)) }
}

impl<A> Debug for Actions<A>
where A: Ord + Debug + Clone
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    let mut out = String::new();
    let mut expected = BTreeSet::new();
    writeln!(out, "let mut set = OrdSet::new();")?;
    for action in &self.0 {
      match action {
        Action::Insert(ref value) => {
          expected.insert(value.clone());
          writeln!(out, "set.insert({:?});", value)?;
        }
        Action::Remove(ref value) => {
          expected.remove(value);
          writeln!(out, "set.remove({:?});", value)?;
        }
      }
    }
    writeln!(
      out,
      "let expected = vec!{:?};",
      expected.into_iter().collect::<Vec<_>>()
    )?;
    writeln!(out, "assert_eq!(OrdSet::from(expected), set);")?;
    write!(f, "{}", super::code_fmt(&out))
  }
}

quickcheck! {
  fn comprehensive(actions: Actions<u8>) -> bool {
    let mut set = OrdSet::new();
    let mut nat = BTreeSet::new();
    let mut res = true;
    for action in actions.0 {
      match action {
        Action::Insert(value) => {
          let len = nat.len() + if nat.contains(&value) {
            0
          } else {
            1
          };
          nat.insert(value);
          set.insert(value);
          res = res && len == set.len();
        }
        Action::Remove(value) => {
          let len = nat.len() - if nat.contains(&value) {
            1
          } else {
            0
          };
          nat.remove(&value);
          set.remove(&value);
          res = res && len == set.len();
        }
      }
      res = res && nat.len() == set.len()
      && OrdSet::from(nat.clone()) == set
      && nat.iter().eq(set.iter());
    }
    res
  }
}
