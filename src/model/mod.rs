
//!
//! models for iron_with_db
//!
//! ## Overview
//!
//! Models are used in order to create a code representation of data. Iron does not come with
//! models, or MVC, built in, so iron_with_db will have one overlaying it instead.
//!
//! -zpallin
//!

pub trait Model {
    fn insert(&self) -> Result<String,String>;
    fn update(&self) -> Result<String,String>;
    fn delete(&self) -> Result<String,String>;
}

pub mod tests;
