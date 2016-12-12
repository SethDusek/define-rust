extern crate serde;
extern crate serde_json;
extern crate hyper;
use dictionaries::{Definition, Dictionary};
use hyper::Client;
use hyper::header::ContentLength;
use std::io::Read;
#[derive(Deserialize, Debug)]
pub struct WordnikDefinition {
    #[serde(rename="textProns")]
    pub text_prons: Vec<Option<String>>,
    #[serde(rename="sourceDictionary")]
    pub source_dictionary: String,
    #[serde(rename="exampleUses")]
    pub example_uses: Vec<Option<String>>,
    #[serde(rename="relatedWords")]
    pub related_words: Vec<Option<String>>,
    pub labels: Vec<Option<String>>,
    pub citations: Vec<Option<String>>,
    pub word: String,
    #[serde(rename="partOfSpeech")]
    pub part_of_speech: String,
    pub sequence: String,
    #[serde(rename="attributionText")]
    pub attribution_text: String,
    pub text: String,
    pub score: i16,
}


pub struct Wordnik {
    pub session: Client,
    pub key: String,
}

impl Wordnik {
    pub fn new<'c>(key: &'c str) -> Wordnik {
        Wordnik {
            session: Client::new(),
            key: key.to_owned(),
        }
    }
}

impl Dictionary for Wordnik {
    fn get_definitions(&mut self, word: &str) -> Result<Vec<Definition>, &str> {
        let url = format!("http://api.wordnik.com:80/v4/word.\
                           json/{word}/definitions?limit=200&includeRelated=true&useCanonical=fal\
                           se&includeTags=false&api_key={key}",
                          word = word,
                          key = self.key);
        let mut response = self.session.get(&url).send().unwrap();
        let cap = response.headers.get::<ContentLength>().map(|v| v.0 as usize).unwrap_or(0);
        let mut body = String::with_capacity(cap as usize);
        response.read_to_string(&mut body);
        let decoded: Vec<WordnikDefinition> = serde_json::from_str(&body).unwrap();
        let definitions = decoded.iter()
                                 .map(|value| {
                                     Definition {
                                         word: word.to_string(),
                                         text: value.text.clone(),
                                     }
                                 })
                                 .collect();
        if decoded.len() == 0 {
            return Err("No definitions");
        }
        Ok(definitions)
    }

    fn clone_to_box(&self) -> Box<Dictionary> {
        Box::new(self.clone())
    }
}

impl Clone for Wordnik {
    fn clone(&self) -> Self {
        Wordnik {
            key: self.key.clone(),
            session: Client::new(),
        }
    }
}

unsafe impl Send for Wordnik {}
