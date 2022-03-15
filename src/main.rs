mod file;
mod utils;

use file::position::Postion;

fn main() {
    let p = Postion::new("".to_string(), 2, 3);
    println!("{}", p.to_string());
}
