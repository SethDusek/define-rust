#![allow(dead_code)]
extern crate serde;
extern crate serde_json;
extern crate hyper;

use dictionaries::traits::{Definition, Dictionary};
use hyper::Client;
use hyper::header::ContentLength;
use std::io::Read;

#[derive(Deserialize)]
struct UrbanDefinition {
    defid: u32,
    word: String,
    author: String,
    permalink: String,
    definition: String,
    example: Option<String>,
    thumbs_up: Option<i32>,
    thumbs_down: Option<i32>,
    current_vote: Option<String>
}
#[derive(Deserialize)]
struct Response {
    tags: Option<Vec<String>>,
    result_type: Option<String>,
    sounds: Option<Vec<String>>,
    list: Vec<UrbanDefinition>
}

pub struct Urban {
    session: Client,
    pub key: String
}

impl Urban {
    pub fn new(key: &str) -> Urban {
        Urban {key: key.to_owned(), session: Client::new()}
    }
}

impl Dictionary for Urban {
    fn get_definitions(&mut self, word: &str) -> Result<Vec<Definition>, &str> {
        let url = format!("https://mashape-community-urban-dictionary.p.mashape.com/define?term={}", word);
        let mut headers = hyper::header::Headers::new();
        headers.set_raw("X-Mashape-Key", vec![self.key.clone().into()]);
        let mut request = self.session.get(&url)
            .send().unwrap();
        let len: usize = request.headers.get::<ContentLength>().map(|v| v.0 as usize).unwrap_or(0);
        let mut response_string = String::with_capacity(len);
        request.read_to_string(&mut response_string);
        let response: Response = serde_json::from_str(&response_string).unwrap();
        let mut definitions: Vec<Definition> = Vec::new();
        for definition in response.list {
            definitions.push(Definition { word: definition.word.to_owned(), text: definition.definition.to_owned() });
        }
        if definitions.len()>0 {
            Ok(definitions)
        }
        else {
            Err("Couldn't find any definitions")
        }
    }

    fn clone_to_box(&self) -> Box<Dictionary> { Box::new(self.clone()) }
}

impl Clone for Urban {
    fn clone(&self) -> Self {
        Urban {key: self.key.clone(), session: Client::new()}
    }
}

unsafe impl Send for Urban {}
