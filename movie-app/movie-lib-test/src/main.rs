extern crate movie_lib;

use movie_lib::movies::play;

fn main() {
    println!("inside main of test");
    play("学习".to_string());
}
