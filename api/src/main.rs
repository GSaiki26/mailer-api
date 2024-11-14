// Libs
use utils::get_settings;

mod utils;

// Functions
fn main() {
    println!("{:?}", get_settings().unwrap());
}
