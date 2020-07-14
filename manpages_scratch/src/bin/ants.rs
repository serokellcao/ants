extern crate ants;

use production::exports as prod;
use ants::hex_directions::*;
use ants::hex_cartography::*;

fn test_map_cell() {
  println!("Here are some map tokens:");
  for token in map_tokens_iter() {
    println!("{:?}", token);
  }
  println!("Here is the bijective mapping between enum and characters:");
  println!("{:?}", map_tokens());
}

fn test_directions() {
  let p33 = Pos(3,3);
  let p22 = Pos(2,2);
  for dir in dir_iter() {
    //              This call requires Pos to have Copy semantics vvv
    println!("{:<2} adj of (2, 2): {:?}", format!("{}", dir), adj(p22, dir));
    println!("{:<2} adj of (3, 3): {:?}", format!("{}", dir), adj(p33, dir));
  }
  for sd in sense_dir_iter() {
    for p in [p22, p33].iter() {
      for dir in dir_iter() {
        print!("For ant facing {:<2} in {:?} ", format!("{}", dir), *p);
        print!("{} is {:?}", format!("{}", sd), sense_dir(*p, dir, sd));
        println!("");
      }
    }
  }
}

fn main() {
  println!("{}, World!", prod::it_exports());
  test_directions();
  test_map_cell();
}
