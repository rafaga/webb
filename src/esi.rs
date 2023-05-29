use rfesi::prelude::*;
use tokio::time::{Instant, timeout_at};
use tokio::time::Duration;
use std::net::SocketAddr;
use hyper::Server;
use crate::auth_service::MakeSvc;
use crate::objects::{Character, Alliance, Corporation};
use chrono::{DateTime,NaiveDateTime};
use chrono::Utc;
use std::path::Path;
use rusqlite::*;
use uuid::Uuid;
use tokio::task;
use futures::executor::block_on;


pub struct EsiManager<'a>{
    pub esi: Esi,
    pub characters: Vec<Character>,
    pub path: &'a Path,
    uuid: Uuid,
}

impl<'a> EsiManager<'a> {

    fn open_flags() -> OpenFlags {
        let mut flags = OpenFlags::default();
        flags.set(OpenFlags::SQLITE_OPEN_NO_MUTEX, false);
        flags.set(OpenFlags::SQLITE_OPEN_FULL_MUTEX, true);
        flags
    }

    fn migrate_database(self) -> Result<bool,Error> {
        Ok(true)
    }

    pub fn del_characters(self, characters: Vec<Character>) -> Result<bool,Error> {
        let conn = Connection::open_with_flags(self.path, EsiManager::open_flags())?;
        let query = String::from(["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat());
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;

        let query = String::from("DELETE FROM eveCharacter WHERE characterId = ?;");
        for player in characters {
            let mut statement = conn.prepare(query.as_str())?;
            statement.execute(rusqlite::params![player.id])?;
        }
        Ok(true)
    }

    pub fn add_characters(self, characters: Vec<Character>) -> Result<bool,Error> {
        let conn = Connection::open_with_flags(self.path, EsiManager::open_flags())?;
        let query = String::from(["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat());
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;

        let mut query = String::from("INSERT INTO char (characterId,");
        query += "name,corporation,alliance,portrait,lastLogon) VALUES (?,?,?,?,?,?)";
        for player in characters {
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
            statement.execute(params)?;
            if let Some(auth_data) = player.auth {
                query = String::from("INSERT INTO charAuth (CharacterId, owner, jti, token) VALUES  (?,?,?,?)");
                let mut statement = conn.prepare(query.as_str())?;
                let values = (auth_data.jti,auth_data.token);
                let params = rusqlite::params![player.id,values.0,values.1,0];
                statement.execute(params)?;
            }
        }
        Ok(true)
    }


    pub fn get_characters(self) -> Result<Vec<Character>,Error> {
        let conn = Connection::open_with_flags(self.path, EsiManager::open_flags())?;
        let query = String::from(["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat());
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;

        let mut result = Vec::new();
        let mut query = String::from("SELECT characterId,name,corp,alliance,");
        query += "portrait,lastLogon FROM char";
        let mut statement = conn.prepare(query.as_str())?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let dt = DateTime::parse_from_rfc3339(row.get::<usize,String>(5)?.as_str());
            let utc_dt = DateTime::from_utc(dt.unwrap().naive_utc(),Utc);
            let mut char = Character::new();
            let corp = Corporation{
                id: row.get(2)?,
                name: String::new(),
            };
            let alliance = Alliance{
                id: row.get(3)?,
                name: String::new(),
            };
            char.id             = row.get(0)?;
            char.name           = row.get(1)?;
            char.corp           = Some(corp);
            char.alliance       = Some(alliance);
            char.last_logon     = utc_dt;
            result.push(char);
        }
        Ok(result)
    }

    pub fn update_characters(&self, characters: Vec<Character>) -> Result<bool,Error> {
        let conn = Connection::open_with_flags(self.path, EsiManager::open_flags())?;
        let mut query = String::from(["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat());
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;

        query = String::from("UPDATE eveCharacter SET name = ?, alliance = ?, corp = ?, ");
        query += "lastlogon = ? WHERE characterId = ?;";
        for player in characters {
            let mut statement = conn.prepare(query.as_str()).unwrap();
            let params = rusqlite::params![player.name,
                                       player.alliance.unwrap().id,
                                       player.corp.unwrap().id,
                                       player.last_logon.to_string(),
                                       player.id];
            statement.execute(params)?;
        }
        Ok(true)
    }

    fn create_database(&self) -> Result<bool,Error> {
        let conn = Connection::open_with_flags(self.path, EsiManager::open_flags())?;
        let mut query = String::from(["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat());
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

    pub fn new(useragent: &str, client_id: &str, client_secret: &str, callback_url: &str, scope: Vec<&str>, database_path: Option<&'a str>) -> Self {

        let esi = EsiBuilder::new()
            .user_agent(useragent)
            .client_id(client_id)
            .client_secret(client_secret)
            .callback_url(callback_url)
            .scope(scope.join(" ").as_str())
            .build().unwrap();

        let path;
        if let Some(pathz) = database_path {
            path = Path::new(pathz);
        } else {
            path = Path::new("telescope.db");
        }

        let obj = EsiManager {
            esi,
            characters: Vec::new(),
            path,
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, "telescope".as_bytes()),
        };

        if !path.exists() {
            // TODO: migration database schema goes here
            //obj.migrate_database();
            if let Err(e) = obj.create_database() {
                panic!("Error: {}", e);
            }
        }
        
        obj
    }

    #[tokio::main]
    pub async fn auth_user(&mut self,port: u16) -> Result<Option<Character>, Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = ([127, 0, 0, 1], port).into();
        let (tx, rx) = tokio::sync::oneshot::channel::<(String,String)>();
        crate::SHARED_TX.lock().await.replace(tx);
        let mut result = (String::new(),String::new());
        let server = Server::bind(&addr)
            .serve(MakeSvc::new())
            .with_graceful_shutdown(async {
                let msg = rx.await.ok();
                if let Some(values) = msg {
                    result = values;
                }
            });
        
        if let Err(err) = timeout_at(Instant::now() + Duration::from_secs(300), server).await {
            eprintln!("{}",err);
            return Ok(None);
        };
        let claims = self.esi.authenticate(result.0.as_str()).await?;      

        let mut player = Character::new();  
        let data = claims.unwrap();
        //character name
        player.name = data.name;
        //character id
        let split:Vec<&str> = data.sub.split(":").collect();
        player.id = split[2].parse::<u64>().unwrap();
        if player.auth != None {
            // owner
            player.auth.as_mut().unwrap().owner = data.owner;
            //jti
            player.auth.as_mut().unwrap().jti= data.jti;
            //expiration Date
            let naive_datetime = NaiveDateTime::from_timestamp_opt(data.exp, 0);
            player.auth.as_mut().unwrap().expiration = Some(DateTime::from_utc(naive_datetime.unwrap(), Utc));
            self.esi.update_spec().await?;        
            let public_info = self.esi.group_character().get_public_info(player.id).await?;
            let corp_info = self.esi.group_corporation().get_public_info(public_info.corporation_id).await?;
            let corp = Corporation{
                id: public_info.corporation_id,
                name: corp_info.name,
            };
            player.corp = Some(corp);
            let ally_info = self.esi.group_alliance().get_info(public_info.alliance_id).await?;
            let ally = Alliance {
                id: public_info.alliance_id,
                name: ally_info.name,
            };
            player.alliance = Some(ally);
            let player_portraits = self.esi.group_character().get_portrait(player.id).await?;
            player.photo = Some(player_portraits.px64x64);
        }
        Ok(Some(player))
    }
   
}