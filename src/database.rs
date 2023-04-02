use std::path::Path;
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
        query = String::from("CREATE TABLE eveCharacter (characterId INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL,");
        query += " corporationId INTEGER NOT NULL, allianceId INTEGER NOT NULL, token VARCHAR(255) NOT NULL";
        query += " lastLogon DATETIME NOT NULL, expiration DATETIME)";
        conn.execute(query.as_str(),())?;
        query = String::from("CREATE TABLE metadata (id INTEGER PRIMARY KEY,value VARCHAR(255) NOT NULL);");
        conn.execute(query.as_str(),())?;
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
        query += "name,corporationId,allianceId,code,lastLogon,token) VALUES (?,?,?,?,?,?)";

        let mut statement = conn.prepare(query.as_str())?;
        let dt = character.last_logon.to_rfc3339();
        statement.execute(rusqlite::params![character.id,character.name,character.corporation,character.alliance,dt,character.token])?;
        Ok(true)
    }

    pub fn get_characters(self) -> Result<Vec<Character>,Error> {
        let mut result = Vec::new();
        if let None = self.connection{
            return Ok(result)
        }
        let conn = self.connection.unwrap();
        let mut query = String::from("SELECT characterId,name,corporationId,allianceId,");
        query += "token,lastLogon, expiration FROM eveCharacter";
        let mut statement = conn.prepare(query.as_str())?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let dt = DateTime::parse_from_rfc3339(row.get::<usize,String>(5)?.as_str());
            let char = Character{
                id: row.get(0)?,
                name:row.get(1)?,
                corporation:row.get(2)?,
                alliance:row.get(3)?,
                token: row.get(4)?,
                last_logon:dt.unwrap(),
                expiration: None,
            };
            result.push(char);
        }
        Ok(result)
    }

    pub fn update_character(self,characters: Vec<Character>) -> Result<bool,Error> {
        let conn;
        if let Some(tconn) = self.connection{
            conn = tconn;
        } else {
            return Ok(false);
        }
        for char in characters {
            let mut query = String::from("UPDATE eveCharacter SET name=?,corporationId=?,allianceId=?,");
            query += "token=?,lastlogon=?,expiration=? FROM eveCharacter WHERE characterId=?";
            let mut statement = conn.prepare(query.as_str())?;
            let mut rows = statement.query([char.name,char.corporation.to_string(),char.alliance.to_string(),char.token.to_string(),char.last_logon.to_string(),char.expiration.unwrap().to_string()])?;
        }
        Ok(true)
    }
    
}

impl Default for Database{
    fn default() -> Self{
        Database::new("telescope.db".to_string())
    }
}


pub struct Character {
    pub id: usize,
    pub name: String,
    pub corporation: usize,
    pub alliance: usize,
    pub token:String,
    pub last_logon: DateTime<FixedOffset>,
    pub expiration: Option<DateTime<FixedOffset>>,
}

impl Character{
    pub fn new() -> Self {
        Character { 
            id: 0, 
            name: String::new(), 
            corporation: 0, 
            alliance: 0, 
            token: String::new(), 
            last_logon: DateTime::default(), 
            expiration: None 
        }
    }
}
