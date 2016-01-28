extern crate serde;
extern crate serde_json;
extern crate curl;
use dictionaries::traits::{Definition, Dictionary};
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
    pub score: i16
}



pub struct Wordnik <'a> {
    pub session: curl::http::Handle,
    pub key: &'a str
}

impl <'a> Wordnik <'a> {
    pub fn new<'c>(key: &'c str) -> Wordnik {
        Wordnik { session: curl::http::handle(), key: key }
    }
}

impl <'a> Dictionary for Wordnik <'a> {
    fn get_definitions(&mut self, word: &str) -> Result<Vec<Definition>,&str> {
        let url = format!("http://api.wordnik.com:80/v4/word.json/{word}/definitions?limit=200&includeRelated=true&useCanonical=false&includeTags=false&api_key={key}", word = word, key = self.key);
        let request = self.session.get(&url[..]).exec().unwrap();
        let body = String::from_utf8_lossy(request.get_body());
        let decoded: Vec<WordnikDefinition> = serde_json::from_str(&body).unwrap();
        let definitions = decoded.iter().map(|value| { Definition { word: word.to_string(), text: value.text.clone() } } ).collect();
        if decoded.len()==0 {
            return Err("No definitions")
        }
        Ok(definitions)
    }
}

