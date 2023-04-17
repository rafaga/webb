use chrono::prelude::*;

pub enum TelescopeDbError{
    NoConnection,
}

#[derive(Clone,PartialEq)]
pub struct AuthData{
    pub owner: String,
    pub jti: String,
    pub token:String,
    pub expiration: Option<DateTime<Utc>>,
}


#[derive(Clone,PartialEq)]
pub struct Character {
    pub id: u64,
    pub name: String,
    pub last_logon: DateTime<Utc>,
    pub auth: Option<AuthData>,
    pub corp: Option<Corporation>,
    pub alliance: Option<Alliance>,
    pub photo: Option<String>,
}
#[derive(Clone,PartialEq)]
pub struct Corporation {
    pub id: u64,
    pub name: String,
}

#[derive(Clone,PartialEq)]
pub struct Alliance {
    pub id: u64,
    pub name: String,
}

impl Character{
    pub fn new() -> Self {
        let auth = AuthData {
            owner: String::new(),
            jti: String::new(),
            token: String::new(),
            expiration: None,
        };

        Character { 
            id: 0, 
            name: String::new(),
            last_logon: DateTime::default(), 
            auth: Some(auth),
            corp: None,
            alliance: None,
            photo: None
        }
    }
}
