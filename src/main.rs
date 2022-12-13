use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> std::io::Result<()> {
    // use serde_json::to_writer;
    // use tl::{parse, Bytes, ParserOptions};
    let file = File::open("cetenfolhautf8")?;
    let mut contents = String::new();

    let buf = BufReader::new(file);
    let mut map: HashMap<String, i32> = HashMap::with_capacity(100000);
    let mut count = 0;
    let options = tl::ParserOptions::default();
    for line in buf.lines() {
        match line.unwrap() {
            s if s.starts_with("<ext id=") => {
                // inicio
                // começa a string
                contents.clear();
                contents.push_str(&s);
            }
            s if s.starts_with("</ext>") => {
                // final
                // termina a string e roda o parser
                // zera a string
                contents.push_str(&s);
                let dom = tl::parse(&contents, options).unwrap();
                let parser = dom.parser();
                let element = dom.nodes();

                let text = element[0].inner_text(parser);
                let words = text.split(" ");
                for token in words {
                    let mut key = String::from(token);
                    key.retain(|c| !r#"«»(),".;:'"#.contains(c));
                    // inserir as palavras no hash
                    // contar as palavras
                    if map.contains_key(&key) {
                        *map.get_mut(&key).unwrap() += 1;
                    } else {
                        // println!("{}", &key);
                        map.insert(key, 1);
                    };
                }
                // println!("{:#?}", text);

                // println!("{}", contents);
                // let text = element.inner_text(parser);
                // TODO remover caracteres especiais
            }
            s => {
                // só concatena a string
                contents.push_str(&s);
            }
        };
    }

    // TODO separar o arquivo em partes
    // iterar por linha
    // conferir se a linha começa com tag ext
    // conferir se ela termina com ext
    // fazer o parse do html dessa parte dentro das tags
    // rodar a lógica de contar palavras

    let mut output = File::create("out.txt")?;
    serde_json::to_writer(output, &map)?;

    for (key, value) in map {
        println!("{key}, {value}");
    }
    Ok(())
}
