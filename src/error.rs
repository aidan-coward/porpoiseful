use std::result;

use failure::Fail;


pub type Result<T> = result::Result<T, Error>;

