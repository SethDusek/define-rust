extern crate curl;
extern crate serde;
extern crate serde_json;
extern crate getopts;
extern crate define;
use define::dictionaries::{Dictionary, wordnik};
use define::thesaureses::Thesaurus;
use std::env;
use getopts::{Matches, Options};

static KEY: &'static str = "a2a73e7b926c924fad7001ca3111acd55af2ffabf50eb4ae5";

fn parse_args() -> Matches {
    let argv: Vec<String> = env::args().collect();
    let mut parser = Options::new();
    parser.optflag("t", "--thesaurus", "Finds synonyms for word");
    parser.parse(&argv[1..]).unwrap()
}

fn print_definition<'a, T: Dictionary>(dict: &'a mut T, word: &str) -> Result<(), &'a str> {
    let definitions = try!(dict.get_definitions(word));
    println!("{}", definitions[0].text);
    Ok(())
}

fn print_synonyms<'a, T: Thesaurus>(thes: &'a mut T, word: &str) -> Result<(), &'a str> {
    let synonyms = try!(thes.get_synonyms(word));
    println!("{}", synonyms.join(", "));
    Ok(())
}
fn main() {
    let mut wordnik = wordnik::Wordnik::new(KEY);
    let args = parse_args();
    for word in args.free.iter() {
        println!("{}:", word.to_uppercase());
        print_definition(&mut wordnik, word).unwrap_or_else(|err| println!("{}", err));
        if args.opt_present("t") {
            println!("SYNONYMS:");
            print_synonyms(&mut wordnik, word).unwrap_or_else(|err| println!("{}", err));
        }
    }
}
