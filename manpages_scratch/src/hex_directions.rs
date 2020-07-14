use std::fmt::Display;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use crate::hex_cartography::*;

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
      None => unreachable!(), // Will never execute, because we % 6!
      Some(z) => z,
    },
    Right => match FromPrimitive::from_i8((dir as i8 + 1) % 6) {
      None => unreachable!(),
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

pub fn adj(Pos(x, y) : Pos, d : Dir) -> Pos {
  use Dir::*;
  match (d, even(y)) {
    (E, _)      => Pos(x + 1, y    ),
    (W, _)      => Pos(x - 1, y    ),
    (SE, true)  => Pos(x,     y + 1),
    (SE, false) => Pos(x + 1, y + 1),
    (SW, true)  => Pos(x - 1, y + 1),
    (SW, false) => Pos(x,     y + 1),
    (NW, true)  => Pos(x - 1, y - 1),
    (NW, false) => Pos(x,     y - 1),
    (NE, true)  => Pos(x,     y - 1),
    (NE, false) => Pos(x + 1, y - 1),
  }
}

pub fn adj_result(p : Pos, d : Dir) -> Result<Pos, ()> {
  match adj(p, d) {
    x if x.0 < 0 || x.1 < 0 => Err(()),
    x => Ok(x),
  }
}

#[inline]
pub fn even< I : From<i8> +
                 std::ops::BitAnd<Output = I> +
                 PartialEq >
           (x : I) -> bool {
  x & I::from(1) == I::from(0)
}

/*
#[inline]
pub fn odd< I : From<i8> +
                std::ops::BitAnd<Output = I> +
                PartialEq >
           (x : I) -> bool {
  x & I::from(1) == I::from(1)
}
*/

#[cfg(test)]
mod tests {
  use super::*;
  use crate::hex_cartography::Pos;

  #[test]
  fn directions_are_calculated_correctly_with_adj() {
    use Dir::*;
    let p11 = Pos(1,1);
    let p22 = Pos(2,2);

    /*

      0,0    1,0    2,0    3,0

         0,1    1,1    2,1    3,1

      0,2    1,2    2,2    3,2

         0,3    1,3    2,3    3,3

    */

    assert_eq!(adj(p22, E), Pos(3,2));
    assert_eq!(adj(p22, SE), Pos(2,3));
    assert_eq!(adj(p22, SW), Pos(1,3));
    assert_eq!(adj(p22, W), Pos(1,2));
    assert_eq!(adj(p22, NW), Pos(1,1));
    assert_eq!(adj(p22, NE), Pos(2,1));

    assert_eq!(adj(p11, E), Pos(2,1));
    assert_eq!(adj(p11, SE), Pos(2,2));
    assert_eq!(adj(p11, SW), Pos(1,2));
    assert_eq!(adj(p11, W), Pos(0,1));
    assert_eq!(adj(p11, NW), Pos(1,0));
    assert_eq!(adj(p11, NE), Pos(2,0));

  }

  #[test]
  fn directions_are_calculated_safely_with_adj_result() {
    use Dir::*;
    let p00 = Pos(0,0);

    assert_eq!(adj_result(p00, SE), Ok(Pos(0,1)));
    assert_eq!(adj_result(p00, E), Ok(Pos(1,0)));
    assert_eq!(adj_result(p00, NE), Err(()));
    assert_eq!(adj_result(p00, NW), Err(()));
    assert_eq!(adj_result(p00, W), Err(()));
    assert_eq!(adj_result(p00, SW), Err(()));
  }

}
