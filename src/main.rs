extern crate curl;
extern crate serde;
extern crate serde_json;
extern crate getopts;
extern crate define;
use define::dictionaries::{Dictionary, Definition, wordnik};
use std::env;
use getopts::{Matches, Options};

static KEY: &'static str = "a2a73e7b926c924fad7001ca3111acd55af2ffabf50eb4ae5";

fn parse_args() -> Matches {
    let argv: Vec<String> = env::args().collect();
    let parser = Options::new();
    parser.parse(&argv[1..]).unwrap()
}

fn print_definition<'a, T: Dictionary>(dict: &'a mut T, word: &str) -> Result<(), &'a str> {
    let definitions = try!(dict.get_definitions(word));
    println!("{}", definitions[0].text);
    Ok(())
}
fn main() {
    let mut wordnik = wordnik::Wordnik::new(KEY);
    let args = parse_args();
    for word in args.free.iter() {
        println!("{}:", word.to_uppercase());
        let definition = print_definition(&mut wordnik, word, None).unwrap_or_else(|err| { println!("{}", err) } );
    }
}
