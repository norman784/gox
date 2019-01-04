extern crate gox;

use std::{
    fs::File,
    io::Result,
};

use gox::{Gox, Only};

fn main() -> Result<()> {
    let file = format!("{}/assets/example.gox", env!("CARGO_MANIFEST_DIR"));
    let mut stream = File::open(&file).unwrap();
    let gox = Gox::new(&mut stream, vec![Only::Layers, Only::Blocks]);
    println!("{:#?}", gox);

    Ok(())
}