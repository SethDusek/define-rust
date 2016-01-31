extern crate curl;
extern crate serde;
extern crate serde_json;
extern crate getopts;
extern crate define;
use define::dictionaries::{Dictionary, wordnik};
use define::thesaureses::Thesaurus;
use std::collections::HashMap;
use std::env;
use getopts::{Matches, Options};

static KEY: &'static str = "a2a73e7b926c924fad7001ca3111acd55af2ffabf50eb4ae5";

fn parse_args() -> (Options, Matches) {
    let argv: Vec<String> = env::args().collect();
    let mut parser = Options::new();
    parser.optflag("t", "thesaurus", "Finds synonyms for word");
    let parsed = parser.parse(&argv[1..]).unwrap();
    (parser, parsed)
}

fn get_sources()
    -> (HashMap<String, Box<Dictionary>>,
        HashMap<String, Box<Thesaurus>>)
{
    // insert your dictionaries here
    let mut dictionaries: HashMap<String, Box<Dictionary>> = HashMap::new();
    let mut wordnik = wordnik::Wordnik::new(KEY);
    dictionaries.insert(String::from("wordnik"), Box::new(wordnik.clone()));
    dictionaries.insert(String::from("example"), Box::new(define::dictionaries::example::ExampleDictionary));
    let mut thesaureses: HashMap<String, Box<Thesaurus>> = HashMap::new();
    thesaureses.insert(String::from("wordnik"), Box::new(wordnik.clone()));
    (dictionaries, thesaureses)
}
fn print_definition<'a, T: Dictionary + ?Sized>(dict: &'a mut Box<T>,
                                                word: &str)
                                                -> Result<(), &'a str> {
    let definitions = try!(dict.get_definitions(word));
    println!("{}", definitions[0].text);
    Ok(())
}

fn print_synonyms<'a, T: Thesaurus + ?Sized>(thes: &'a mut Box<T>, word: &str) -> Result<(), &'a str> {
    let synonyms = try!(thes.get_synonyms(word));
    println!("{}", synonyms.join(", "));
    Ok(())
}
fn main() {
    let (mut dictionaries, mut thesaureses) = get_sources();
    let mut dictionary = dictionaries.get_mut("wordnik").unwrap();
    let mut thesaurus = thesaureses.get_mut("wordnik").unwrap();
    let (opts, args) = parse_args(); //we needs opts too for printing brief description
    if args.free.is_empty() {
        println!("{}", opts.usage("USAGE: define WORD"));
    }
    for word in &args.free {
        println!("{}:", word.to_uppercase());
        print_definition(dictionary, word).unwrap_or_else(|err| println!("{}", err));
        if args.opt_present("t") {
            println!("SYNONYMS:");
            print_synonyms(thesaurus, word).unwrap_or_else(|err| println!("{}", err));
        }
    }
}
