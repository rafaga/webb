use chrono::{DateTime,Utc};
use rusqlite::{Connection,OpenFlags};
use crate::objects::{Character,Corporation,Alliance};
use crate::esi::Error;
use std::path::Path;
use uuid::Uuid;

pub(crate) struct PlayerDatabase {

}

impl PlayerDatabase{

    pub(crate) fn create_database(path: &Path,uuid: Uuid) -> Result<bool,Error> {
        let conn = Connection::open_with_flags(path, PlayerDatabase::open_flags())?;
        let mut query = ["PRAGMA key = '",uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
        //Character Public Data
        query = String::from("CREATE TABLE char (characterId INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL,");
        query += " corporation TEXT NOT NULL, alliance TEXT NOT NULL, portrait BLOB,";
        query += " lastLogon DATETIME NOT NULL)";
        let mut statement = conn.prepare(query.as_str())?;
        statement.execute([])?;
        // Character Auth Data
        query = String::from("CREATE TABLE charAuth (characterId INTEGER REFERENCES char (characterId)");
        query += " ON UPDATE CASCADE ON DELETE CASCADE, owner TEXT NOT NULL, jti TEXT NOT NULL, ";
        query += " token VARCHAR(255) NOT NULL, expiration DATETIME)";
        let mut statement = conn.prepare(query.as_str())?;
        statement.execute([])?;
        // Corporations
        let mut query = "CREATE TABLE corp (corpId INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL)";
        let mut statement = conn.prepare(query)?;
        statement.execute([])?;
        // Alliances
        query = "CREATE TABLE alliance (allianceId INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL)";
        statement = conn.prepare(query)?;
        statement.execute([])?;
        // Telescope Metadata
        query = "CREATE TABLE metadata (id VARCHAR(255) PRIMARY KEY,value VARCHAR(255) NOT NULL);";
        statement = conn.prepare(query)?;
        statement.execute([])?;
        query = "INSERT INTO metadata (id,value) VALUES (?,?)";
        statement = conn.prepare(query)?;
        statement.execute(["db","0"])?;
        Ok(true)
    }

    pub(crate) fn select_characters(conn: &Connection, ids: Vec<u64>) -> Result<Vec<Character>,Error> {
        let mut result = Vec::new();
        let mut query = String::from("SELECT characterId,name,corp,alliance,portrait,lastLogon FROM char");
        if !ids.is_empty() {
            let vars = PlayerDatabase::repeat_vars(ids.len());
            query = format!("SELECT characterId, name, corporation, alliance, portrait, lastLogon FROM char WHERE characterId IN ({})", vars);
        }
        let mut statement = conn.prepare(&query)?;
        let mut rows = statement.query(rusqlite::params_from_iter(ids))?;
        while let Some(row) = rows.next()? {
            let dt = DateTime::parse_from_rfc3339(row.get::<usize,String>(5)?.as_str());
            let utc_dt = DateTime::from_utc(dt.unwrap().naive_utc(),Utc);
            let mut char = Character::new();
            char.id             = row.get(0)?;
            char.name           = row.get(1)?;
            char.corp = if let Ok(value) = row.get::<usize,u64>(2){
                Some(Corporation{
                    id: value,
                    name: String::new(),
                })
            } else {
                None
            };
            char.alliance = if let Ok(value) = row.get::<usize,u64>(3){
                Some(Alliance{
                    id: value,
                    name: String::new(),
                })
            } else {
                None
            };
            char.last_logon     = utc_dt;
            result.push(char);
        }
        Ok(result)
    }
    
    // Updated
    pub(crate) fn update_character(conn: &Connection, character: Character) -> Result<usize,Error> {
        let mut query = String::from("UPDATE char SET name = ?, alliance = ?, corporation = ?, ");
        query += "lastlogon = ? WHERE characterId = ?;";
        let mut statement = conn.prepare(query.as_str()).unwrap();
        let params = rusqlite::params![character.name,
                                    character.alliance.unwrap().id,
                                    character.corp.unwrap().id,
                                    character.last_logon.to_string(),
                                    character.id];
        let rows:usize = statement.execute(params)?;
        Ok(rows)
    }
    
    pub(crate) fn insert_character(conn: &Connection, player: Character) -> Result<usize,Error> {
        let mut query = String::from("INSERT INTO char (characterId,");
        query += "name,corporation,alliance,portrait,lastLogon) VALUES (?,?,?,?,?,?)";
        let mut statement = conn.prepare(query.as_str())?;
        let dt = player.last_logon.to_rfc3339();
        let corp = match player.corp {
            None => 0,
            Some(t_corp) => t_corp.id,
        };
        let alliance = match player.alliance {
            None => 0,
            Some(t_alliance) => t_alliance.id,
        };
        let params = rusqlite::params![player.id,player.name,corp,alliance,"0",dt];
        let rows = statement.execute(params)?;
        if let Some(auth_data) = player.auth {
            query = String::from("INSERT INTO charAuth (CharacterId, owner, jti, token) VALUES  (?,?,?,?)");
            let mut statement = conn.prepare(query.as_str())?;
            let values = (auth_data.jti,auth_data.token);
            let params = rusqlite::params![player.id,values.0,values.1,0];
            let _ = statement.execute(params)?;
        }
        Ok(rows)
    }
    
    fn repeat_vars(count: usize) -> String {
        assert_ne!(count, 0);
        let mut s = "?,".repeat(count);
        // Remove trailing comma
        s.pop();
        s
    }

    pub(crate) fn open_flags() -> OpenFlags {
        let mut flags = OpenFlags::default();
        flags.set(OpenFlags::SQLITE_OPEN_NO_MUTEX, false);
        flags.set(OpenFlags::SQLITE_OPEN_FULL_MUTEX, true);
        flags
    }
    
    pub(crate) fn migrate_database() -> Result<bool,Error> {
        Ok(true)
    }
    
    pub(crate) fn del_characters(conn: &Connection, ids: Vec<u64>) -> Result<usize,Error> {
        if !ids.is_empty() {
            let vars = PlayerDatabase::repeat_vars(ids.len());
            let query = format!("DELETE FROM char WHERE characterId IN ({})", vars);
            let mut statement = conn.prepare(&query)?;
            if let Ok(rows) = statement.execute(rusqlite::params_from_iter(ids)){
                Ok(rows)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }
}
