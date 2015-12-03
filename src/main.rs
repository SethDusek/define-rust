extern crate hyper;
extern crate getopts;
extern crate serde;
extern crate serde_json;
use std::io::Read;
use std::env;
use hyper::Client;
use serde_json::Value;
use getopts::Options;
fn main() {
    let wordclient = Client::new();
    let args: Vec<String> = env::args().collect();
    let parser = Options::new();
    let opts = parser.parse(&args[1..]).unwrap();
    for word in opts.free {
        let url = "http://api.wordnik.com/v4/word.json/".to_string()+&word+"/definitions?limit=200&includeRelated=true&useCanonical=false&includeTags=false&api_key=a2a73e7b926c924fad7001ca3111acd55af2ffabf50eb4ae5";
        let mut resp = wordclient.get(&url).send().unwrap();
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();
        let word: Value = serde_json::from_str(&body).unwrap();
        println!("{}",word.as_array().unwrap()[0].as_object().unwrap().get("text").unwrap().as_string().unwrap());
    }
}
