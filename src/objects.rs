use chrono::prelude::*;
use rusqlite::Error;

pub trait EsiObject {
    fn retrieve() -> Result<bool, Error>;
}

pub enum TelescopeDbError {
    NoConnection,
}

#[derive(Clone, PartialEq)]
pub struct AuthData {
    pub token: String,
    pub expiration: Option<DateTime<Utc>>,
    pub refresh_token:String,
}

impl AuthData{
    pub fn new() -> Self {
        AuthData {
            token: String::new(),
            expiration: None,
            refresh_token: String::new()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub last_logon: DateTime<Utc>,
    pub corp: Option<Corporation>,
    pub alliance: Option<Alliance>,
    pub photo: Option<String>,
    pub location: i32,
}

impl Character {
    pub fn new() -> Self {
        Character {
            id: 0,
            name: String::new(),
            last_logon: DateTime::default(),
            corp: None,
            alliance: None,
            photo: None,
            location: 0,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Corporation {
    pub id: i32,
    pub name: String,
}

impl Corporation {
    pub fn new() -> Self {
        Corporation {
            id: 0,
            name: String::new(),
        }
    }
}

impl Default for Corporation {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicCatalog for Corporation {
    type Output = i32;

    fn id(&self) -> Self::Output {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Alliance {
    pub id: i32,
    pub name: String,
}

impl Alliance {
    pub fn new() -> Self {
        Alliance {
            id: 0,
            name: String::new(),
        }
    }
}

impl Default for Alliance {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicCatalog for Alliance {
    type Output = i32;

    fn id(&self) -> Self::Output {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub trait BasicCatalog {
    type Output;

    fn id(&self) -> Self::Output;
    fn name(&self) -> &str;
}
