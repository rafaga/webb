use std::path::Path;
use rusqlite::*;

pub struct Database{
    pub version: i64,
    pub path: String,
}

impl Database{
    pub fn new() -> Self{
        Database {  
            version: 0,
            path: String::new(),
        }
    }

    pub fn regenerate() -> () {
        
    }

    pub fn open() -> () {

    }
}

impl Default for Database{
    fn default() -> Self{
        Database::new()
    }
}

