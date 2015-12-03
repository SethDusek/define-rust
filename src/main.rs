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
fn main() {
    let wordclient = Client::new();
    let args: Vec<String> = env::args().collect();
    let parser = Options::new();
    let opts = parser.parse(&args[1..]).unwrap();
    for word in opts.free {
        println!("{}:",word.to_uppercase());
        let url = format!("http://api.wordnik.com/v4/word.json/{word}/definitions?api_key={key}",key = KEY, word = word);
        println!("{}",url);
        let mut resp = wordclient.get(&url).send().unwrap();
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();
        println!("{}",body);
        let word: Value = serde_json::from_str(&body).unwrap();
        println!("{}",word.as_array().unwrap()[0].as_object().unwrap().get("text").unwrap().as_string().unwrap());
    }
}
