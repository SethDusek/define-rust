extern crate hyper;
extern crate getopts;
extern crate serde;
extern crate serde_json;
use std::io::Read;
use std::env;
use hyper::Client;
use serde_json::Value;
use getopts::Options;
const KEY: &'static str = "1e940957819058fe3ec7c59d43c09504b400110db7faa0509";
struct Dict<'a> {
    key: &'a str,
    httpclient: Client
}
impl <'a> Dict <'a> {
    fn new(key: &str) -> Dict {
        Dict {key: key, httpclient: Client::new()}
    }
    fn get_definition(&self,word: &str) -> Vec<Value>  {
        let url = format!("http://api.wordnik.com/v4/word.json/{word}/definitions?api_key={key}",key = self.key, word = word);
        let mut resp = self.httpclient.get(&url).send().expect("Failed to send request");
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();
        let word: Value = serde_json::from_str(&body).unwrap();
        word.as_array().unwrap().clone()
    }
}
fn main() {
    let wordclient: Dict = Dict::new(KEY);
    let args: Vec<String> = env::args().collect();
    let parser = Options::new();
    let opts = parser.parse(&args[1..]).unwrap();
    for word in opts.free {
        println!("{}:",word.to_uppercase());
        println!("{}",wordclient.get_definition(&word)[0].as_object().unwrap().get("text").unwrap().as_string().unwrap());
    }
}
