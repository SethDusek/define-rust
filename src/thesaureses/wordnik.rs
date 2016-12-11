extern crate serde_json;
extern crate hyper;
use thesaureses::Thesaurus;
pub use dictionaries::wordnik::Wordnik;
use hyper::header::ContentLength;
use std::io::Read;
#[derive(Deserialize)]
pub struct Synonym {
    #[serde(rename="relationshipType")]
    relationship_type: String,
    words: Vec<String>,
}
impl Thesaurus for Wordnik {
    fn get_synonyms(&mut self, word: &str) -> Result<Vec<String>, &str> {
        let url = format!("http://api.wordnik.com:80/v4/word.\
                           json/{word}/relatedWords?useCanonical=false&relationshipTypes=synonym&\
                           limitPerRelationshipType=10&api_key={key}",
                          word = word,
                          key = self.key);
        let mut request = self.session.get(&url).send().unwrap();
        let len = request.headers.get::<ContentLength>().map(|v| v.0 as usize).unwrap_or(0);
        let mut body = String::with_capacity(len);
        request.read_to_string(&mut body).unwrap();
        let decoded: Vec<Synonym> = serde_json::from_str(&body).unwrap();
        if decoded.len() == 0 {
            return Err("No synonyms found");
        }
        Ok(decoded[0].words.clone())
    }
}
