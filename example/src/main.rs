extern crate gox;

use std::fs::File;
use gox::{Gox, Only};

fn main() {
    let file = format!("{}/assets/example.gox", env!("CARGO_MANIFEST_DIR"));
    let stream = &(File::open(&file).unwrap());
    let gox = Gox::new(stream, vec![Only::Image]);
    println!("{:#?}", gox);
}