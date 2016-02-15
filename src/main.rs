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
static UKEY: &'static str = "ub2JDDg9Iumsh1HfdO3a3HQbZi0up1qe8LkjsnWQvyVvQLFn1q";

struct Config {
    max_definitions: i16,
}

fn parse_args() -> (Options, Matches) {
    let argv: Vec<String> = env::args().collect();
    let mut parser = Options::new();
    parser.optflag("t", "thesaurus", "Finds synonyms for word");
    parser.optflag("u", "urban", "Urban Dictionary");
    parser.optopt("", "source", "Set dictionary source", "SOURCE");
    parser.optopt("",
                  "thesaurus-source",
                  "Set thesaurus source (not functional yet)",
                  "SOURCE");
    let parsed = parser.parse(&argv[1..]).unwrap();
    (parser, parsed)
}

fn get_sources()
    -> (HashMap<String, Box<Dictionary>>,
        HashMap<String, Box<Thesaurus>>)
{
    // insert your dictionaries here
    let mut dictionaries: HashMap<String, Box<Dictionary>> = HashMap::new();
    let wordnik = wordnik::Wordnik::new(KEY);
    dictionaries.insert(String::from("wordnik"), Box::new(wordnik.clone()));
    dictionaries.insert(String::from("example"),
                        Box::new(define::dictionaries::example::ExampleDictionary));
    dictionaries.insert(String::from("urban"),
                        Box::new(define::dictionaries::urban::Urban::new(UKEY)));
    let mut thesaureses: HashMap<String, Box<Thesaurus>> = HashMap::new();
    thesaureses.insert(String::from("wordnik"), Box::new(wordnik.clone()));
    (dictionaries, thesaureses)
}

fn print_definition<'a, T: Dictionary + ?Sized>(dict: &'a mut Box<T>,
                                                word: &str,
                                                max_definitions: Option<usize>)
                                                -> Result<(), &'a str> {
    let definitions = try!(dict.get_definitions(word));
    let max_definitions = max_definitions.unwrap_or(1);
    if max_definitions > 1 {
        for (wordnumber, word) in definitions.iter().take(max_definitions).enumerate() {
            println!("{}. {}", wordnumber + 1, word.text);
        }
    } else {
        println!("{}", definitions[0].text);
    }
    Ok(())
}

fn print_synonyms<'a, T: Thesaurus + ?Sized>(thes: &'a mut Box<T>,
                                             word: &str)
                                             -> Result<(), &'a str> {
    let synonyms = try!(thes.get_synonyms(word));
    println!("{}", synonyms.join(", "));
    Ok(())
}

fn get_source<'a, T: Dictionary + ?Sized, K: Thesaurus + ?Sized>
    (dictionaries: &'a mut HashMap<String, Box<T>>,
     thesaureses: &'a mut HashMap<String, Box<K>>,
     args: &Matches)
     -> (&'a mut Box<T>, &'a mut Box<K>) {

    let mut source: String = "wordnik".to_owned();
    let tsource: String;
    tsource = args.opt_str("thesaurus-source").unwrap_or("wordnik".to_owned()).to_owned();
    if !args.opt_present("source") {
        for dictionary_source in dictionaries.keys() {
            if args.opt_defined(&dictionary_source) && args.opt_present(&dictionary_source) {
                source = dictionary_source.clone();
            }
        }
    }
    else if args.opt_present("source") {
        source = args.opt_str("source").unwrap_or("wordnik".to_owned()).to_owned();
    }
    let dict: Option<&mut Box<T>> = dictionaries.get_mut(&source);
    let thes: Option<&mut Box<K>> = thesaureses.get_mut(&tsource);
    (dict.unwrap(), thes.unwrap())
}



fn main() {
    let (mut dictionaries, mut thesaureses) = get_sources();
    let (opts, args) = parse_args(); //we needs opts too for printing brief description
    let (dictionary, thesaurus) = get_source(&mut dictionaries, &mut thesaureses, &args);
    if args.free.is_empty() {
        println!("{}", opts.usage("USAGE: define WORD"));
    }
    for word in &args.free {
        println!("{}:", word.to_uppercase());
        print_definition(dictionary, &word.to_lowercase(), Some(3)).unwrap_or_else(|err| println!("{}", err));
        if args.opt_present("t") {
            println!("SYNONYMS:");
            print_synonyms(thesaurus, word).unwrap_or_else(|err| println!("{}", err));
        }
    }
}
