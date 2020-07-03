use std::fmt::Display;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Display)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(FromPrimitive)]
pub enum Dir {
  E = 0,
  SE = 1,
  SW = 2,
  W = 3,
  NW = 4,
  NE = 5,
}

#[derive(Debug)]
pub struct WrappedDir(pub Option<Dir>);

/*
impl WrappedDir {
  fn new() -> WrappedDir {
    WrappedDir(None)
  }
}
*/

impl Iterator for WrappedDir {
  type Item = Dir;

  fn next(&mut self) -> Option<Dir> {

    fn update_and_return(wd : &mut WrappedDir, dir : Option<Dir>) -> Option<Dir> {
      match dir {
        None => None,
        Some(dir) => {
          wd.0 = Some(dir);
          Some(dir)
        },
      }
    }

    fn opt_dir_to_i8(d : Option<Dir>) -> i8 {
      match d {
        None    => -1,
        Some(x) => x as i8,
      }
    }

    fn i8_to_opt_dir(x : i8) -> Option<Dir> {
      match FromPrimitive::from_i8(x) {
        None => None,
        y    => y,
      }
    }

    let x = opt_dir_to_i8(self.0);
    if Dir::NE as i8 == x {
      None
    } else {
      update_and_return(self, i8_to_opt_dir(x + 1))
    }

  }
}

#[derive(Debug)]
pub struct Pos(pub i32, pub i32);

pub fn adj(Pos(x, y) : Pos, d : Dir) -> Pos {
  use Dir::*;
  match (d, even(y)) {
    (E, _)      => Pos(x + 1, y    ),
    (W, _)      => Pos(x - 1, y    ),
    (SE, true)  => Pos(x,     y + 1),
    (SE, false) => Pos(x + 1, y + 1),
    (SW, true)  => Pos(x - 1, y    ),
    (SW, false) => Pos(x,     y + 1),
    (NW, true)  => Pos(x - 1, y - 1),
    (NW, false) => Pos(x,     y - 1),
    (NE, true)  => Pos(x,     y - 1),
    (NE, false) => Pos(x + 1, y - 1),
  }
}

#[inline]
pub fn even(x : i32) -> bool {
  x & 1 == 0
}

/*
#[inline]
pub fn odd(x : i32) -> bool {
  x & 1 == 1
}
*/
