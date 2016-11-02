
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

use core::iter::{FromIterator, IntoIterator};

/// Datatype is used by db plugins to provide context for primitives
/// for data entry.
pub trait DataType {

    /// the format string for the database
    /// for example, in mysql, you might want for a String's impl to look like:
    ///
    ///   pub fn db_storage_format(&self) -> String {
    ///       "TEXT".to_string()
    ///   }
    ///
    /// Or something along those lines.
    ///
    /// Each db will designate their own storage formats to return, and some might
    /// not return any.
    ///
    /// For custom implementations, you can re-impl the trait locally
    fn db_storage_format(&self) -> String;

}


/// Model struct for handling model schema for a db
#[derive(Debug)]
pub struct Model<T>
    where T: IntoIterator {

    /// must be an iterator of some sort
    pub data: T,
}

impl<T> Model<T>
    where T: IntoIterator {

    fn new(data: T) -> Model<T> {

        Model {
            data: data,
        }
    }
}

/// Trait for model interface with database
pub trait ModelInterface {
    fn insert<K, V, T, E> (&self, key: K, value: V) -> Result<T, E> 
        where V: DataType;
    fn delete<K, T, E> (&self, K) -> Result<T, E>;
}

