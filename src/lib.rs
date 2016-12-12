#![feature(proc_macro, custom_derive, custom_attribute)]
#![feature(box_syntax)]
#[macro_use] extern crate serde_derive;
extern crate hyper;
pub mod dictionaries;
pub mod thesaureses;
pub use dictionaries::{Definition, Dictionary};
pub use thesaureses::Thesaurus;
