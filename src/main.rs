mod file;
mod utils;

use file::position::InFilePosition;

fn main() {
    let p = InFilePosition::new("".to_string(), 2, 3);
    println!("{}", p.to_string());
}
