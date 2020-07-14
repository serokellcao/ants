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

// TODO: should be aware of map boundaries!
pub fn adj_maybe(p : Pos, d : Dir) -> Option<Pos> {
  if !p.is_valid() {
    None
  } else {
    match adj(p, d) {
      x if x.is_valid() => Some(x),
      _ => None,
    }
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
  use Dir::*;
  use crate::hex_cartography::Pos;

  #[test]
  fn directions_are_calculated_correctly_with_adj() {
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
  fn directions_are_calculated_safely_with_adj_maybe() {
    let p00 = Pos(0,0);

    assert_eq!(adj_maybe(p00, SE), Some(Pos(0,1)));
    assert_eq!(adj_maybe(p00, E), Some(Pos(1,0)));
    assert_eq!(adj_maybe(p00, NE), None);
    assert_eq!(adj_maybe(p00, NW), None);
    assert_eq!(adj_maybe(p00, W), None);
    assert_eq!(adj_maybe(p00, SW), None);
  }

  #[quickcheck]
  fn south_and_north_adj_always_increase_and_decrease_y(p : Pos) -> bool {
    let ne = adj_maybe(p, NE);
    let se = adj_maybe(p, SE);
    let nw = adj_maybe(p, NW);
    let sw = adj_maybe(p, SW);
    let err = None;

    print!(".");

    if p.is_inner() {
      ne.unwrap().1 == p.1 - 1 &&
      nw.unwrap().1 == p.1 - 1 &&
      se.unwrap().1 == p.1 + 1 &&
      sw.unwrap().1 == p.1 + 1
    } else {
      if !p.is_valid() {
        [ne, se, nw, sw].iter().all(|x| *x == err)
      } else if p.1 == Pos::max() {
        [se, sw].iter().all(|x| *x == err)
      } else if p.1 == 0 {
        [ne, nw].iter().all(|x| *x == err)
      } else {
        true
      }
    }
  }

  #[quickcheck]
  fn east_and_west_adj_always_increase_and_decrease_x(p : Pos) -> bool {
    let e = adj_maybe(p, E);
    let w = adj_maybe(p, W);
    let err = None;

    if p.is_inner() {
      e.unwrap().0 == p.0 + 1 &&
      w.unwrap().0 == p.0 - 1
    } else if p.0 == Pos::max() {
      e == err
    } else if p.0 == 0 {
      w == err
    } else {
      true
    }
  }

  #[quickcheck]
  fn east_adj_diagonals_and_west_adj_diagonals_at_most_preserve_x(p : Pos) -> bool {
    let ne = adj_maybe(p, NE);
    let se = adj_maybe(p, SE);
    let nw = adj_maybe(p, NW);
    let sw = adj_maybe(p, SW);

    if p.is_inner() {
      ne.unwrap().0 == se.unwrap().0 &&
      nw.unwrap().0 == sw.unwrap().0 &&
      (ne.unwrap().0 == p.0 || ne.unwrap().0 == p.0 + 1) &&
      (nw.unwrap().0 == p.0 || nw.unwrap().0 == p.0 - 1)
    } else {
      true
    }
  }

}
