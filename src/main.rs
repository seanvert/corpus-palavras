use std::fs::File;
use std::io::prelude::*;
use tl::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("testedados")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let dom = tl::parse(&mut contents, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom
        .get_element_by_id("8")
        .expect("Failed to find element")
        .get(parser)
        .unwrap();
    println!("{:?}", element);

    Ok(())
}
