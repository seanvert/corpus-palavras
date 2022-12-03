use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::string::ToString;

fn main() -> std::io::Result<()> {
    // use serde_json::to_writer;
    // use tl::{parse, Bytes, ParserOptions};
    let mut file = File::open("testedados")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let dom = tl::parse(&mut contents, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let mut map: HashMap<String, i32> = HashMap::with_capacity(10000);
    // separar os fragmentos
    for n in 1..10 {
        let string = String::from(n.to_string()).into_bytes();
        let mut input = tl::Bytes::new();
        input.set(string);
        let element = dom
            .get_element_by_id(input)
            .expect("Failed to find element")
            .get(parser)
            .unwrap();
        // separar o texto
        let text = element.inner_text(parser);
        // TODO remover caracteres especiais
        let words = text.split(" ");
        // separar as palavras
        for token in words {
            let key = String::from(token);
            // inserir as palavras no hash
            // contar as palavras
            if map.contains_key(&key) {
                *map.get_mut(&key).unwrap() += 1;
            } else {
                map.insert(key, 1);
            };
        }
    }
    // salvar arquivo
    let mut output = File::create("out.txt")?;
    serde_json::to_writer(output, &map)?;

    for (key, value) in map {
        println!("{key}, {value}");
    }
    Ok(())
}
