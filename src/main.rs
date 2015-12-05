extern crate hyper;
extern crate getopts;
extern crate serde;
extern crate serde_json;
use std::io::Read;
use std::env;
use hyper::Client;
use std::collections::btree_map::BTreeMap;
use serde_json::Value;
use getopts::Options;
const KEY: &'static str = "1e940957819058fe3ec7c59d43c09504b400110db7faa0509";
const TKEY: &'static str = "e415520c671c26518df498d8f4736cac";
struct Dict<'a> {
    key: &'a str,
    tkey: &'a str,
    httpclient: Client
}
impl <'a> Dict <'a> {
    fn new<'c> (key: &'c str,tkey: &'c str) -> Dict<'c> {
        Dict {key: key, httpclient: Client::new(),tkey:tkey}
    }
    fn get_definitions(&self,word: &str) -> Result<Vec<String>,&str> {
        let url = format!("http://api.wordnik.com/v4/word.json/{word}/definitions?api_key={key}",key = self.key, word = word);
        let mut resp = self.httpclient.get(&url).send().expect("Failed to send request");
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();
        let word: Value = serde_json::from_str(&body).unwrap();
        let decoded = word.as_array().unwrap().clone();
        if decoded.len()==0 {
            return Err("No definitions found")
        }
        let mut definitions = Vec::new();
        for word in &decoded {
            definitions.push(word.as_object().unwrap().get("text").unwrap().as_string().unwrap().to_string());
        }
        Ok(definitions)
    }
    fn get_thesaurus(&self,word: &str) -> Result<BTreeMap<String,Value>,&str> {
        let url = format!("http://words.bighugelabs.com/api/2/{key}/{word}/json", key = self.tkey, word = word);
        let mut resp = self.httpclient.get(&url).send().expect("Failed to get thesaurus");
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();
        if &body == "" { 
            return Err("No synonyms found")
        }
        let mut synonyms: BTreeMap<String,Value> = serde_json::from_str(&body).unwrap();
        synonyms = synonyms.clone();
        Ok(synonyms)
    }
}

fn main() {
    let wordclient: Dict = Dict::new(KEY,TKEY);
    let args: Vec<String> = env::args().collect();
    let mut parser = Options::new();
    parser.optflag("t","thesaurus","use thesaurus");
    let opts = parser.parse(&args[1..]).unwrap();
    for word in &opts.free {
        println!("{}:",word.to_uppercase());
        let definitions = wordclient.get_definitions(word);
        match definitions {
            Ok(defs) => {println!("{}",defs[0]);},
            Err(err) => {println!("{}",err);}
        };
        if opts.opt_present("t") {
            let syns = wordclient.get_thesaurus(&word);
            match syns {
                Ok(map) => {
                   if map.get("noun").is_some() {
                        let mut nounstr = String::new();
                        let mut first: bool = true;
                        for noun in map.get("noun").unwrap().as_object().unwrap().get("syn").unwrap().as_array().unwrap() {
                            if first {
                                nounstr = nounstr + &(noun.as_string().unwrap());
                                first = false;
                            }
                            else {
                                nounstr = nounstr + ", " + &(noun.as_string().unwrap());
                            }
                   }
                       println!("NOUNS:\n{}",nounstr);
                   }
                },
                Err(err) => {println!("{}",err);
            }
        }}

    }
}
