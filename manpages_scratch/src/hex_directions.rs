use std::fmt::Display;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy)]
#[derive(Display, FromPrimitive)]
pub enum Dir {
  E = 0,
  SE = 1,
  SW = 2,
  W = 3,
  NW = 4,
  NE = 5,
}
pub fn dir_iter() -> impl Iterator<Item=Dir> { /* TODO: make it into a generic funciton */
  (0..6).map(|x| FromPrimitive::from_i8(x).unwrap())
}

#[derive(Debug, Clone, Copy)]
#[derive(Display, FromPrimitive)]
pub enum LR {
  Left = 0,
  Right = 1,
}
/*
pub fn lr_iter() -> impl Iterator<Item=LR> {
  (0..2).map(|x| FromPrimitive::from_i8(x).unwrap())
}
*/

#[derive(Debug, Clone, Copy)]
#[derive(Display, FromPrimitive)]
pub enum SenseDir {
  Here = 0,
  Ahead = 1,
  LeftAhead = 2,
  RightAhead = 3,
}
pub fn sense_dir_iter() -> impl Iterator<Item=SenseDir> {
  (0..4).map(|x| FromPrimitive::from_i8(x).unwrap())
}

#[inline]
pub fn turn(lr : LR, dir : Dir) -> Dir {
  use LR::*;
  match lr {
    Left  => match FromPrimitive::from_i8((dir as i8 + 5) % 6) {
      None => dir, // Will never execute, because we % 6!
      Some(z) => z,
    },
    Right => match FromPrimitive::from_i8((dir as i8 + 1) % 6) {
      None => dir,
      Some(z) => z,
    },
  }
}

pub fn sense_dir(cell : Pos, dir : Dir, sd : SenseDir) -> Pos {
  use LR::*;
  use SenseDir::*;
  match sd {
    Here       => cell,
    Ahead      => adj(cell, dir),
    LeftAhead  => adj(cell, turn(Left, dir)),
    RightAhead => adj(cell, turn(Right, dir)),
  }
}

#[derive(Debug, Clone, Copy)]
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
