#![feature(box_syntax)]
extern crate curl;
extern crate serde;
extern crate num_cpus;
extern crate serde_json;
extern crate getopts;
extern crate crossbeam;
extern crate define;
use define::dictionaries::{Dictionary, Definition, wordnik};
use define::thesaureses::Thesaurus;
use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use std::sync::mpsc;
use getopts::{Matches, Options};


static KEY: &'static str = "a2a73e7b926c924fad7001ca3111acd55af2ffabf50eb4ae5";
static UKEY: &'static str = "ub2JDDg9Iumsh1HfdO3a3HQbZi0up1qe8LkjsnWQvyVvQLFn1q";
const THREAD_ENABLED: bool = true;
struct Config {
    max_definitions: i16,
}

fn parse_args() -> (Options, Matches) {
    let argv: Vec<String> = { let mut args: Vec<String> = env::args().collect(); args.dedup(); args };
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
    -> (HashMap<String, Box<Dictionary + Send>>,
        HashMap<String, Box<Thesaurus>>)
{
    // insert your dictionaries here
    let mut dictionaries: HashMap<String, Box<Dictionary + Send>> = HashMap::new();
    let wordnik = wordnik::Wordnik::new(KEY);
    dictionaries.insert(String::from("wordnik"), box wordnik.clone());
    dictionaries.insert(String::from("example"),
                        box define::dictionaries::example::ExampleDictionary);
    dictionaries.insert(String::from("urban"),
                        box define::dictionaries::urban::Urban::new(UKEY));
    let mut thesaureses: HashMap<String, Box<Thesaurus>> = HashMap::new();
    thesaureses.insert(String::from("wordnik"), box wordnik.clone());
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
    if ! THREAD_ENABLED {
        for word in &args.free {
            println!("{}:", word.to_uppercase());
            print_definition(dictionary, &word.to_lowercase(), Some(3)).unwrap_or_else(|err| println!("{}", err));
            if args.opt_present("t") {
                println!("SYNONYMS:");
                print_synonyms(thesaurus, word).unwrap_or_else(|err| println!("{}", err));
            }
        }
    }
    else {
        let free_clone = args.free.clone();
        let dict_arc = Arc::new(dictionary.clone_to_box());
        //chunks.map(|chunk| chunks_vec.push(chunk));
        let (tx, rx) = mpsc::channel::<Vec<Definition>>();
        let mut words: Vec<&[String]> = Vec::new();
        let threads = num_cpus::get();;
        {
            let threads = num_cpus::get();
            for chunk in free_clone.chunks((free_clone.len() + threads - 1) / threads) {
                words.push(chunk);
            }
        }
        let words_arc = Arc::new(words);
        for num in 0..words_arc.len() {
            let dict = dict_arc.clone();
            let tx = tx.clone();
            let words = words_arc.clone()[num];
            unsafe {
            crossbeam::spawn_unsafe(move || {
                let mut dictionary = dict.clone_to_box();
                let mut definitions = Vec::new();
                for word in words.iter() {
                    let defs = dictionary.get_definitions(word).unwrap_or(Vec::new());
                    for definition in &defs {
                        definitions.push(definition.clone());
                    }
                }
                tx.send(definitions);
            });
            }
        }   
        let mut definitions: HashMap<String, Vec<Definition>> = HashMap::new();
        for _ in 0..words_arc.len() { 
            let thread_definitions = rx.recv().unwrap();
            for definition in thread_definitions {
                if definitions.contains_key(&definition.word) {
                    definitions.get_mut(&definition.word).unwrap().push(definition);
                }

                else {
                    definitions.insert(definition.word.clone().to_lowercase(), vec![definition]); //lowercase is necessary because some dictionaries return the definitions in different cases
                }
            }
        }
        for (word, definitions) in definitions.iter() {
                println!("{}:", word.to_uppercase());
                for (number, definition) in definitions.iter().take(3).enumerate() {
                    println!("{}. {}", number + 1, definition.text);
                }
            }
    }
}
