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

pub fn dir_iter() -> impl Iterator<Item=Dir> {
  (0..6).map(|x| FromPrimitive::from_i8(x).unwrap())
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
