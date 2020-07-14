use bimap::BiMap;
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
pub struct Pos(pub i8, pub i8);

impl Arbitrary for Pos {
  fn arbitrary<G: Gen>(g: &mut G) -> Pos {
    Pos(
      std::cmp::max(i8::arbitrary(g), 1),
      std::cmp::max(i8::arbitrary(g), 1)
    )
  }
}

fn digit_to_char<I : From<u8> + ToString>(x : I) -> char {
  let s = x.to_string();
  match s.chars().next() {
    Some(y) => y,
    _       => unreachable!(),
  }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
// TODO think about nomenclature to translate maps into world state more
// directly.
pub enum MapToken {
  Rock,
  Clear,
  // TODO should be generalized as Home(Red | Black)
  RedHome,
  BlackHome,
  Food(u8),
}
pub fn map_tokens_zip() -> Vec<char> {
    let mut simple = vec!['#', '.', '+', '-'];
    let range : Vec<char> = (0..10).map(digit_to_char).collect();
    simple.extend(range);
    simple
}
pub fn map_tokens_iter() -> impl Iterator<Item=(MapToken, char)> {
  use MapToken::*;
  let mtz = map_tokens_zip();
  (0..14).map(move |x| match x {
    0 => (Rock, mtz[0]),
    1 => (Clear, mtz[1]),
    2 => (RedHome, mtz[2]),
    3 => (BlackHome, mtz[3]),
    x => (Food(x - 4), mtz[x as usize]),
  })
}
pub fn map_tokens() -> BiMap<MapToken, char> {
  let mut mts = BiMap::new();
  for x in map_tokens_iter() {
    mts.insert(x.0, x.1);
  }
  mts
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn map_tokens_biject_onto_map_encoding_chars_as_specified() {
    use MapToken::*;
    /*
    # rocky cell
    . clear cell (containing nothing interesting)
    + red anthill cell
    - black anthill cell
    1 to 9 clear cell containing the given number of food particles
    */
    let mt = map_tokens();
    assert_eq!(mt.get_by_left(&Rock), Some(&'#'));
    assert_eq!(mt.get_by_left(&Clear), Some(&'.'));
    assert_eq!(mt.get_by_left(&RedHome), Some(&'+'));
    assert_eq!(mt.get_by_left(&BlackHome), Some(&'-'));
    for x in 0..10 {
      assert_eq!(mt.get_by_left(&Food(x)), Some(&digit_to_char(x)));
    }
    for (x, y) in map_tokens_iter() {
      assert_eq!(
        &x,
        mt.get_by_right(&mt.get_by_left(&x).unwrap()).
          unwrap()
      );
      assert_eq!(
        &y,
        mt.get_by_left(&mt.get_by_right(&y).unwrap()).
          unwrap()
      );
    }
  }
}
