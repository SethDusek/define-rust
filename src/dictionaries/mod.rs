pub mod wordnik;
pub mod example;
pub mod urban;

#[derive(Clone, Debug)]
pub struct Definition {
    pub text: String,
    pub word: String,
}

pub trait Dictionary: Send {
    fn get_definitions(&mut self, &str) -> ::Result<Vec<Definition>>;
    fn clone_to_box(&self) -> Box<Dictionary>;
}
