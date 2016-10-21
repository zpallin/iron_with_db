
//! primary crate for iron_with_db
//! 
//! ## Overview
//! 
//! Iron with db implements the iron webserver with a flexible db backend that
//! allows for easy mvc development.
//!
//! The immediate goal is to have a fully functional mongodb backend.
//!
//! In the future I will implement more dbs with common syntax for calls.
//!
//! - zpallin
//!

extern crate iron;
extern crate router;
extern crate persistent;

pub use iron::prelude::*;
pub use iron::{BeforeMiddleware, AfterMiddleware, status};
pub use iron::typemap::Key;
pub use router::Router;
pub use persistent::{Read};

pub mod db;

