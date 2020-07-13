use bimap::BiMap;

#[derive(Debug, Clone, Copy)]
pub struct Pos(pub u8, pub u8);

fn digit_to_char<I : From<u8> + ToString>(x : I) -> char {
  let s = x.to_string();
  match s.chars().next() {
    Some(y) => y,
    _       => unreachable!(),
  }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
pub enum MapToken {
  Rock,
  Clear,
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
