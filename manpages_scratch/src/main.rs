use production::exports as prod;
mod hex_grid;
use hex_grid::*;

#[macro_use]
extern crate enum_display_derive;

fn main() {
    println!("{}, World!", prod::it_exports());
    for dir in dir_iter() {
      println!("{:<2} adj of (2, 2): {:?}", format!("{}", dir), a=adj(Pos(2, 2), dir));
      println!("{:<2} adj of (3, 3): {:?}", format!("{}", dir), adj(Pos(3, 3), dir));
    }
}
