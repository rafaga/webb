use chrono::{DateTime,Utc};
use rusqlite::{Connection,OpenFlags,ToSql};
use crate::objects::{Character,Corporation,Alliance, BasicCatalog};
use crate::esi::Error;
use std::path::Path;
#[cfg(feature = "crypted-db")]
use uuid::Uuid;

pub(crate) struct PlayerDatabase {
}

impl PlayerDatabase{

    #[cfg(feature = "crypted-db")]
    pub(crate)fn crypted_database_open( conn: &Connection) -> Result<(),Error> {
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, "telescope".as_bytes());
        let query = ["PRAGMA key = '",uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
        Ok(())
    }

    pub(crate) fn create_database(path: &Path) -> Result<bool,Error> {
        let conn = Connection::open_with_flags(path, PlayerDatabase::open_flags())?;
        #[cfg(feature = "crypted-db")]
        PlayerDatabase::crypted_database_open(&conn)?;
        
        //Character Public Data
        let mut query = String::from("CREATE TABLE char (id INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL,");
        query += " corporation INTEGER REFERENCES corp(id) ON DELETE CASCADE ON UPDATE CASCADE,";
        query += " alliance INTEGER REFERENCES alliance(id) ON DELETE CASCADE ON UPDATE CASCADE,";
        query += " portrait BLOB, lastLogon DATETIME NOT NULL, location INTEGER NOT NULL)";
        let mut statement = conn.prepare(&query)?;
        statement.execute([])?;
        
        // Character Auth Data
        query = String::from("CREATE TABLE char_auth (id INTEGER REFERENCES char (id)");
        query += " ON UPDATE CASCADE ON DELETE CASCADE, owner TEXT NOT NULL, jti TEXT NOT NULL, ";
        query += " token VARCHAR(255) NOT NULL, expiration DATETIME)";
        let mut statement = conn.prepare(&query)?;
        statement.execute([])?;
        
        // Corporations
        let mut query = "CREATE TABLE corp (id INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL)";
        let mut statement = conn.prepare(query)?;
        statement.execute([])?;
        
        // Alliances
        query = "CREATE TABLE alliance (id INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL)";
        statement = conn.prepare(query)?;
        statement.execute([])?;

        // Telescope Metadata
        let mut query = "CREATE TABLE metadata (id VARCHAR(255) PRIMARY KEY,value VARCHAR(255) NOT NULL);";
        statement = conn.prepare(query)?;
        statement.execute([])?;
        query = "INSERT INTO metadata (id,value) VALUES (?,?)";
        statement = conn.prepare(query)?;
        statement.execute(["db","0"])?;
        Ok(true)
    }

    pub(crate) fn select_characters(conn: &Connection, ids: Vec<u64>) -> Result<Vec<Character>,Error> {
        let mut result = Vec::new();
        let mut query = String::from("SELECT id, name, corporation, alliance, portrait, lastLogon, location FROM char");
        if !ids.is_empty() {
            let vars = PlayerDatabase::repeat_vars(ids.len());
            query = format!("SELECT id, name, corporation, alliance, portrait, lastLogon, location FROM char WHERE id IN ({})", vars);
        }
        let mut statement = conn.prepare(&query)?;
        let mut rows = statement.query(rusqlite::params_from_iter(ids))?;
        while let Some(row) = rows.next()? {
            let dt = DateTime::parse_from_rfc3339(row.get::<usize,String>(5)?.as_str());
            let mut char = Character::new();
            char.id             = row.get(0)?;
            char.name           = row.get(1)?;
            char.photo          = row.get(4)?;
            char.corp = if let Ok(value) = row.get::<usize,u64>(2){
                Some(PlayerDatabase::select_corporation(conn, vec![value])?[0].clone())
            } else {
                None
            };
            char.alliance = if let Ok(value) = row.get::<usize,u64>(3){
                Some(PlayerDatabase::select_alliance(conn, vec![value])?[0].clone())
            } else {
                None
            };
            if let Ok(time) = dt {
                let utc_dt = DateTime::from_naive_utc_and_offset(time.naive_utc(),Utc);
                char.last_logon     = utc_dt;
            }
            char.location       = row.get::<usize,u64>(6)?;
            result.push(char);
        }
        Ok(result)
    }
    
    // Updated
    pub(crate) fn update_character(conn: &Connection, character: &Character) -> Result<usize,Error> {
        let mut query = String::from("UPDATE char SET name = ?, alliance = ?, corporation = ?, ");
        query += "lastlogon = ?, location = ? WHERE id = ?;";
        let mut statement = conn.prepare(query.as_str()).unwrap();
        let params = rusqlite::params![character.name,
                                    character.alliance.as_ref().unwrap().id,
                                    character.corp.as_ref().unwrap().id,
                                    character.last_logon.to_string(),
                                    character.location,
                                    character.id];
        let rows:usize = statement.execute(params)?;
        Ok(rows)
    }
    
    pub(crate) fn insert_character(conn: &Connection, player: &Character) -> Result<usize,Error> {
        let mut query = String::from("INSERT INTO char (id,");
        query += "name,corporation,alliance,portrait,lastLogon,location) VALUES (?,?,?,?,?,?,?)";
        let mut statement = conn.prepare(query.as_str())?;
        let dt = player.last_logon.to_rfc3339();
        statement.raw_bind_parameter(1, player.id)?;
        statement.raw_bind_parameter(2, &player.name)?;
        if player.corp .is_some() {
            statement.raw_bind_parameter(3, player.corp.as_ref().unwrap().id)?;
        }
        if player.alliance.is_some() {
            statement.raw_bind_parameter(4, player.alliance.as_ref().unwrap().id)?;
        }
        if player.photo.is_some(){
            statement.raw_bind_parameter(5, player.photo.clone().unwrap())?;
        }
        statement.raw_bind_parameter(6, dt)?;
        statement.raw_bind_parameter(7, player.location)?;
        let rows = statement.raw_execute()?;
        if let Some(auth_data) = &player.auth {
            query = String::from("INSERT INTO char_auth (id, owner, jti, token) VALUES  (?,?,?,?)");
            let mut statement = conn.prepare(query.as_str())?;
            let values = (auth_data.jti.clone(),auth_data.token.clone());
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
    
    pub(crate) fn delete_characters(conn: &Connection, ids: Vec<u64>) -> Result<usize,Error> {
        PlayerDatabase::delete_general(conn, "char", ids)
    }

    // Corporation
    pub(crate) fn select_corporation(conn: &Connection, ids: Vec<u64>) -> Result<Vec<Corporation>,Error> {
        let mut result = Vec::new();
        let mut query = String::from("SELECT id,name FROM corp");
        if !ids.is_empty() {
            let vars = PlayerDatabase::repeat_vars(ids.len());
            query = format!("SELECT id,name FROM corp WHERE id IN ({})", vars);
        }
        let mut statement = conn.prepare(&query)?;
        let mut rows = statement.query(rusqlite::params_from_iter(ids))?;
        while let Some(row) = rows.next()? {
            let corp = Corporation { 
                id: row.get::<usize,u64>(0)?, 
                name: row.get::<usize,String>(1)?
            };
            result.push(corp);
        }
        Ok(result)
    }

    pub(crate) fn update_corporation(conn: &Connection, corp: &Corporation) -> Result<usize,Error> {
        PlayerDatabase::update_catalog(conn, "corp", corp)
    }
    
    pub(crate) fn insert_corporation(conn: &Connection, corp: &Corporation) -> Result<usize,Error> {
        PlayerDatabase::insert_catalog(conn, "corp", corp)
    }

    pub(crate) fn delete_corporation(conn: &Connection, ids: Vec<u64>) -> Result<usize,Error> {
        PlayerDatabase::delete_general(conn, "corp", ids)
    }

    // Alliance
    pub(crate) fn select_alliance(conn: &Connection, ids: Vec<u64>) -> Result<Vec<Alliance>,Error> {
        let mut result = Vec::new();
        let mut query = String::from("SELECT id,name FROM alliance");
        if !ids.is_empty() {
            let vars = PlayerDatabase::repeat_vars(ids.len());
            query = format!("SELECT id,name FROM alliance WHERE id IN ({})", vars);
        }
        let mut statement = conn.prepare(&query)?;
        let mut rows = statement.query(rusqlite::params_from_iter(ids))?;
        while let Some(row) = rows.next()? {
            let ally = Alliance { 
                id: row.get::<usize,u64>(0)?, 
                name: row.get::<usize,String>(1)?
            };
            result.push(ally);
        }
        Ok(result)
    }

    pub(crate) fn update_alliance(conn: &Connection, ally: &Alliance) -> Result<usize,Error> {
        PlayerDatabase::update_catalog(conn, "alliance", ally)
    }
    
    pub(crate) fn insert_alliance(conn: &Connection, ally: &Alliance) -> Result<usize,Error> {
        PlayerDatabase::insert_catalog(conn, "alliance", ally)
    }
    pub(crate) fn delete_alliance(conn: &Connection, ids: Vec<u64>) -> Result<usize,Error> {
        PlayerDatabase::delete_general(conn, "alliance", ids)
    }

    // function to delete values
    fn delete_general(conn: &Connection, table: &str, ids: Vec<u64>) -> Result<usize,Error> {
        if !ids.is_empty() {
            let vars = PlayerDatabase::repeat_vars(ids.len());
            let query = format!("DELETE FROM {} WHERE id IN ({})", table, vars);
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

    // generic Function to insert new values on a catalog
    fn insert_catalog<B: BasicCatalog>(conn: &Connection, table: &str, obj: &B) -> Result<usize,Error> where <B as BasicCatalog>::Output: ToSql {
        let query = format!("INSERT INTO {} (id,name) VALUES (?,?);", table);
        let mut statement = conn.prepare(&query)?;
        let params = rusqlite::params![obj.id(),obj.name()];
        let rows = statement.execute(params)?;
        Ok(rows)
    }

    // generic Function to update values on a catalog
    fn update_catalog<B: BasicCatalog>(conn: &Connection, table: &str, obj: &B) -> Result<usize,Error> where <B as BasicCatalog>::Output: ToSql {
        let query = format!("UPDATE {} SET name = ? WHERE id = ?;", table);
        let mut statement = conn.prepare(&query)?;
        let params = rusqlite::params![obj.name(),obj.id()];
        let rows = statement.execute(params)?;
        Ok(rows)
    }



}
