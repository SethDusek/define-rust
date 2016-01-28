extern crate serde_json;
use thesaureses::Thesaurus;
pub use dictionaries::wordnik::Wordnik;
#[derive(Deserialize)]
pub struct Synonym {
    #[serde(rename="relationshipType")]
    relationship_type: String,
    words: Vec<String>
}
impl <'a> Thesaurus for Wordnik <'a> {
    fn get_synonyms(&mut self, word: &str) -> Result<Vec<String>,&str> {
        let url = format!("http://api.wordnik.com:80/v4/word.json/{word}/relatedWords?useCanonical=false&relationshipTypes=synonym&limitPerRelationshipType=10&api_key={key}", word = word, key = self.key);
        let request = self.session.get(&url[..]).exec().unwrap();
        let body = String::from_utf8_lossy(request.get_body());
        let decoded: Vec<Synonym> = serde_json::from_str(&body).unwrap();
        if decoded.len() == 0 {
            return Err("No synonyms found")
        }
        Ok(decoded[0].words.clone())
    }
}
