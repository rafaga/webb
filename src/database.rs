use std::path::Path;
use rfesi::groups::CharacterPortraitInfo;
use rusqlite::*;
use uuid::Uuid;
use chrono::prelude::*;

pub enum TelescopeDbError{
    NoConnection,
}

pub struct Database{
    pub version: i64,
    pub path: String,
    connection: Option<Connection>,
    uuid: Uuid,
}

impl Database{

    pub fn new(database: String) -> Self{
        Database {  
            version: 0,
            path: database,
            connection: None,
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, "telescope".as_bytes()),
        }
    }

    fn create(&mut self) -> Result<bool,Error> {
        let mut query;
        let conn = self.connection.as_ref().unwrap();
        //Character Public Data
        query = String::from("CREATE TABLE char (characterId INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL,");
        query += " corporation TEXT NOT NULL, alliance TEXT NOT NULL, portrait BLOB,";
        query += " lastLogon DATETIME NOT NULL)";
        conn.execute(query.as_str(),())?;
        // Character Auth Data
        query = String::from("CREATE TABLE charAuth (characterId INTEGER REFERENCES char (characterId)");
        query += " ON UPDATE CASCADAE ON DELETE CASCADE, owner TEXT NOT NULL, jti TEXT NOT NULL, ";
        query += " token VARCHAR(255) NOT NULL expiration DATETIME)";
        conn.execute(query.as_str(),())?;
        // Corporations
        query = String::from("CREATE TABLE corp (corpId INTEGER PRIMARY KEY,");
        query += " name VARCHAR(255) NOT NULL)";
        conn.execute(query.as_str(),())?;
        // Alliances
        query = String::from("CREATE TABLE alliance (allianceId INTEGER PRIMARY KEY,");
        query += " name VARCHAR(255) NOT NULL)";
        conn.execute(query.as_str(),())?;
        // Telescope Metadata
        query = String::from("CREATE TABLE metadata (id VARCHAR(255) PRIMARY KEY,value VARCHAR(255) NOT NULL);");
        conn.execute(query.as_str(),())?;
        let query = "INSERT INTO metadata (id,value) VALUES ('db','0')";
        conn.execute(query,())?;
        Ok(true)
    }

    pub fn open(&mut self) -> Result<bool,Error> {
        let database_path = Path::new(self.path.as_str());
        let mut flags = OpenFlags::default();
        flags.set(OpenFlags::SQLITE_OPEN_NO_MUTEX, false);
        flags.set(OpenFlags::SQLITE_OPEN_FULL_MUTEX, true);
        let connection = Connection::open_with_flags(database_path, flags)?;
        let mut query = String::from("PRAGMA key=");
        query += self.uuid.to_string().as_str();
        self.connection = Some(connection);
        if !database_path.exists() {
            self.create()?;
        }
        Ok(true)
    }

    pub fn add_character(self, character:Character) -> Result<bool,Error> {
        if let None = self.connection{
            return Ok(false)
        }
        let conn = self.connection.unwrap();
        let mut query = String::from("INSERT INTO eveCharacter (characterId,");
        query += "name,owner,jti,code,lastLogon,token) VALUES (?,?,?,?,?,?)";

        let mut statement = conn.prepare(query.as_str())?;
        let dt = character.last_logon.to_rfc3339();
        statement.execute(rusqlite::params![character.id,character.name,dt,character.auth.unwrap().token])?;
        Ok(true)
    }

    pub fn get_characters(self) -> Result<Vec<Character>,Error> {
        let mut result = Vec::new();
        if let None = self.connection{
            return Ok(result)
        }
        let conn = self.connection.unwrap();
        let mut query = String::from("SELECT characterId,name,owner,jti,");
        query += "token,lastLogon, expiration FROM eveCharacter";
        let mut statement = conn.prepare(query.as_str())?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let dt = DateTime::parse_from_rfc3339(row.get::<usize,String>(5)?.as_str());
            let utc_dt = DateTime::from_utc(dt.unwrap().naive_utc(),Utc);
            let mut char = Character::new();
            char.id= row.get(0).unwrap();
            char.name= row.get(1).unwrap();
            if let Some(mut tauth) = char.auth {
                tauth.owner         = row.get(2).unwrap();
                tauth.jti           = row.get(3).unwrap();
                tauth.token         = row.get(4).unwrap();
                tauth.expiration    = None;
                char.auth           = Some(tauth);
            }
            char.last_logon= utc_dt;
            result.push(char);
        }
        Ok(result)
    }

    pub fn update_character(self,char: Character) -> Result<bool,Error> {
        let conn;
        if let Some(tconn) = self.connection{
            conn = tconn;
        } else {
            return Ok(false);
        }
        let mut query = String::from("UPDATE eveCharacter SET name=?,owner=?,jti=?,");
        query += "token=?,lastlogon=?,expiration=? FROM eveCharacter WHERE characterId=?";
        let mut statement = conn.prepare(query.as_str())?;
        let params = [char.name,
                                    char.auth.as_ref().unwrap().owner.to_string(),
                                    char.auth.as_ref().unwrap().jti.to_string().clone(),
                                    char.auth.as_ref().unwrap().token.to_string(),
                                    char.last_logon.to_string(),
                                    char.auth.as_ref().unwrap().expiration.unwrap().to_string()];
        statement.execute(params)?;
        Ok(true)
    }
    
}

impl Default for Database{
    fn default() -> Self{
        Database::new("telescope.db".to_string())
    }
}
#[derive(Clone,PartialEq)]
pub struct AuthData{
    pub owner: String,
    pub jti: String,
    pub token:String,
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct Character {
    pub id: u64,
    pub name: String,
    pub last_logon: DateTime<Utc>,
    pub auth: Option<AuthData>,
    pub corp: Option<Corporation>,
    pub alliance: Option<Alliance>,
    pub photo: Option<String>,
}
#[derive(Clone)]
pub struct Corporation {
    pub id: u64,
    pub name: String,
}

#[derive(Clone)]
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
