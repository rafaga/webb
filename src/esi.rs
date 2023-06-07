use rfesi::prelude::*;
use tokio::time::{Instant, timeout_at};
use tokio::time::Duration;
use std::net::SocketAddr;
use hyper::Server;
use crate::auth_service::MakeSvc;
use crate::objects::{Character,Corporation,Alliance};
use chrono::{DateTime,NaiveDateTime};
use chrono::Utc;
use std::path::Path;
use rusqlite::*;
use uuid::Uuid;


use self::player_database::PlayerDatabase;
pub mod player_database;

pub struct EsiManager<'a>{
    pub esi: Esi,
    pub characters: Vec<Character>,
    pub path: &'a Path,
    uuid: Uuid,
}

impl<'a> EsiManager<'a> {

    // Alliance
    pub fn write_alliance(&mut self, alliance:&Alliance) -> Result<usize,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
    
        let players = PlayerDatabase::select_alliance(&conn, vec![alliance.id])?;
        let rows;
        if !players.is_empty() {
            rows = PlayerDatabase::update_alliance(&conn, alliance)?;
        } else {
            rows = PlayerDatabase::insert_alliance(&conn, alliance)?;
        };
        Ok(rows)
    }

    pub fn read_alliance(&mut self, alliance_vec:Option<Vec<u64>>) -> Result<Vec<Alliance>,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
    
        let result;
        if let Some(id_ally) = alliance_vec {
            result = PlayerDatabase::select_alliance(&conn,id_ally)?;
        } else {
            result = PlayerDatabase::select_alliance(&conn,vec![])?;
        };
        Ok(result)
    }

    pub fn remove_alliance(&mut self, alliance_vec:Option<Vec<u64>>) -> Result<usize,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;

        let result;
        if let Some(id_ally) = alliance_vec {
            result = PlayerDatabase::delete_alliance(&conn,id_ally)?;
        } else {
            result = PlayerDatabase::delete_alliance(&conn,vec![])?;
        }
        Ok(result)
    }

    // Corporation
    pub fn write_corporation(&mut self, corp:&Corporation) -> Result<usize,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
    
        let players = PlayerDatabase::select_alliance(&conn, vec![corp.id])?;
        let rows;
        if !players.is_empty() {
            rows = PlayerDatabase::update_corporation(&conn, corp)?;
        } else {
            rows = PlayerDatabase::insert_corporation(&conn, corp)?;
        };
        Ok(rows)
    }

    pub fn read_corporation(&mut self, corporation_vec:Option<Vec<u64>>) -> Result<Vec<Corporation>,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
    
        let result;
        if let Some(id_corp) = corporation_vec {
            result = PlayerDatabase::select_corporation(&conn,id_corp)?;
        } else {
            result = PlayerDatabase::select_corporation(&conn,vec![])?;
        };
        Ok(result)
    }

    pub fn remove_corporation(&mut self, corporation_vec:Option<Vec<u64>>) -> Result<usize,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;

        let result;
        if let Some(id_ally) = corporation_vec {
            result = PlayerDatabase::delete_corporation(&conn,id_ally)?;
        } else {
            result = PlayerDatabase::delete_corporation(&conn,vec![])?;
        }
        Ok(result)
    }

    //Characters
    pub fn write_character(&mut self, char:&Character) -> Result<usize,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
    
        // first we need to assure that Corporation and alliance existys on database
        if let Some(corp) = &char.corp {
            let _ = self.write_corporation(corp)?;
        }

        if let Some(alliance) = &char.alliance {
            let _ = self.write_alliance(alliance)?;
        }

        let players = PlayerDatabase::select_characters(&conn, vec![char.id])?;
        let rows;
        if !players.is_empty() {
            rows = PlayerDatabase::update_character(&conn, char)?;
        } else {
            rows = PlayerDatabase::insert_character(&conn, char)?;
        };
        
        Ok(rows)
    }

    pub fn read_characters(&mut self, char_vec:Option<Vec<u64>>) -> Result<Vec<Character>,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;
    
        let result;
        if let Some(id_chars) = char_vec {
            result = PlayerDatabase::select_characters(&conn,id_chars)?;
        } else {
            result = PlayerDatabase::select_characters(&conn,vec![])?;
        };
        Ok(result)
    }

    pub fn remove_characters(&mut self, char_vec:Option<Vec<u64>>) -> Result<usize,Error> {
        let conn = Connection::open_with_flags(self.path, PlayerDatabase::open_flags())?;
        let query = ["PRAGMA key = '",self.uuid.to_string().as_str(),"'"].concat();
        let mut statement = conn.prepare(query.as_str())?;
        let _ = statement.query([])?;

        let result;
        if let Some(id_chars) = char_vec {
            result = PlayerDatabase::delete_characters(&conn,id_chars)?;
        } else {
            result = PlayerDatabase::delete_characters(&conn,vec![])?;
        }
        Ok(result)
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

        if !obj.path.exists() {
            // TODO: migration database schema goes here
            let _ = PlayerDatabase::migrate_database();
            if let Err(e) = PlayerDatabase::create_database(obj.path, obj.uuid) {
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
        let split:Vec<&str> = data.sub.split(':').collect();
        player.id = split[2].parse::<u64>().unwrap();
        if player.auth.is_some() {
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
            if let Some(ally_id) = public_info.alliance_id{
                let ally_info = self.esi.group_alliance().get_info(ally_id).await?;
                let ally = Alliance {
                    id: ally_id,
                    name: ally_info.name,
                };
                player.alliance = Some(ally);
            }
            let player_portraits = self.esi.group_character().get_portrait(player.id).await?;
            player.photo = player_portraits.px64x64;
            let location = self.esi.group_location().get_location(player.id).await?;
            player.location = location.solar_system_id;
        }
        self.write_character(&player)?;
        Ok(Some(player))
    }
   
}