use crate::objects::{Alliance, Character, Corporation};
use chrono::{TimeDelta, Utc};
use hyper::body::HttpBody;
use hyper::Client;
use hyper_tls::HttpsConnector;
use rfesi::prelude::*;
use rusqlite::vtab::array;
use rusqlite::*;
use std::path::Path;

#[cfg(feature = "crypted-db")]
use uuid::Uuid;

use self::player_database::PlayerDatabase;
pub mod player_database;

#[derive(Clone)]
pub struct EsiManager {
    pub esi: Esi,
    pub characters: Vec<Character>,
    pub path: String,
    pub active_character: Option<i32>,
}

impl EsiManager {
    pub(crate) fn get_standart_connection(&self) -> Result<Connection, Error> {
        let mut flags = OpenFlags::default();
        flags.set(OpenFlags::SQLITE_OPEN_NO_MUTEX, false);
        flags.set(OpenFlags::SQLITE_OPEN_FULL_MUTEX, true);
        let connection = Connection::open_with_flags(self.path.clone(), flags)?;

        // we add the carray module disguised as rarray in rusqlite
        array::load_module(&connection)?;

        #[cfg(feature = "crypted-db")]
        {
            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, "telescope".as_bytes());
            let query = ["PRAGMA key = '", uuid.to_string().as_str(), "'"].concat();
            let mut statement = connection.prepare(query.as_str())?;
            let _ = statement.query([])?;
        }
        Ok(connection)
    }

    // Alliance
    pub fn write_alliance(&mut self, alliance: &Alliance) -> Result<usize, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_write_alliance");
        let conn = self.get_standart_connection().unwrap();

        let players = PlayerDatabase::select_alliance(&conn, vec![alliance.id])?;
        let rows = if !players.is_empty() {
            PlayerDatabase::update_alliance(&conn, alliance)?
        } else {
            PlayerDatabase::insert_alliance(&conn, alliance)?
        };
        Ok(rows)
    }

    pub fn read_alliance(
        &mut self,
        alliance_vec: Option<Vec<i32>>,
    ) -> Result<Vec<Alliance>, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_read_alliance");
        let conn = self.get_standart_connection().unwrap();

        let result = if let Some(id_ally) = alliance_vec {
            PlayerDatabase::select_alliance(&conn, id_ally)?
        } else {
            PlayerDatabase::select_alliance(&conn, vec![])?
        };
        Ok(result)
    }

    pub fn remove_alliance(&mut self, alliance_vec: Option<Vec<i32>>) -> Result<usize, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_remove_alliance");
        let conn = self.get_standart_connection().unwrap();

        let result = if let Some(id_ally) = alliance_vec {
            PlayerDatabase::delete_alliance(&conn, id_ally)?
        } else {
            PlayerDatabase::delete_alliance(&conn, vec![])?
        };
        Ok(result)
    }

    // Corporation
    pub fn write_corporation(&mut self, corp: &Corporation) -> Result<usize, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_write_corporation");
        let conn = self.get_standart_connection().unwrap();

        let corps = PlayerDatabase::select_corporation(&conn, vec![corp.id])?;
        let rows = if !corps.is_empty() {
            PlayerDatabase::update_corporation(&conn, corp)?
        } else {
            PlayerDatabase::insert_corporation(&conn, corp)?
        };
        Ok(rows)
    }

    pub fn read_corporation(
        &mut self,
        corporation_vec: Option<Vec<i32>>,
    ) -> Result<Vec<Corporation>, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_read_corporation");
        let conn = self.get_standart_connection().unwrap();

        let result = if let Some(id_corp) = corporation_vec {
            PlayerDatabase::select_corporation(&conn, id_corp)?
        } else {
            PlayerDatabase::select_corporation(&conn, vec![])?
        };
        Ok(result)
    }

    pub fn remove_corporation(
        &mut self,
        corporation_vec: Option<Vec<i32>>,
    ) -> Result<usize, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_remove_corporation");
        let conn = self.get_standart_connection().unwrap();

        let result = if let Some(id_ally) = corporation_vec {
            PlayerDatabase::delete_corporation(&conn, id_ally)?
        } else {
            PlayerDatabase::delete_corporation(&conn, vec![])?
        };
        Ok(result)
    }

    //Characters
    pub fn write_character(&mut self, char: &Character) -> Result<usize, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_write_character");

        let conn = self.get_standart_connection().unwrap();

        // first we need to assure that Corporation and alliance existys on database
        if let Some(corp) = &char.corp {
            let _ = self.write_corporation(corp)?;
        }

        if let Some(alliance) = &char.alliance {
            let _ = self.write_alliance(alliance)?;
        }

        let players = PlayerDatabase::select_characters(&conn, vec![char.id])?;
        let rows = if !players.is_empty() {
            PlayerDatabase::update_character(&conn, char)?
        } else {
            PlayerDatabase::insert_character(&conn, char)?
        };
        Ok(rows)
    }

    pub fn read_characters(&mut self, char_vec: Option<Vec<i32>>) -> Result<Vec<Character>, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_read_characters");

        let conn = self.get_standart_connection().unwrap();

        let result;
        if let Some(id_chars) = char_vec {
            result = PlayerDatabase::select_characters(&conn, id_chars)?;
        } else {
            result = PlayerDatabase::select_characters(&conn, vec![])?;
        };
        Ok(result)
    }

    pub fn remove_characters(&mut self, char_vec: Option<Vec<i32>>) -> Result<usize, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_remove_character");
        let conn = self.get_standart_connection().unwrap();

        let result = if let Some(id_chars) = char_vec {
            PlayerDatabase::delete_characters(&conn, id_chars)?
        } else {
            PlayerDatabase::delete_characters(&conn, vec![])?
        };
        Ok(result)
    }

    pub fn new(
        useragent: &str,
        client_id: &str,
        _client_secret: &str,
        callback_url: &str,
        scope: Vec<&str>,
        database_path: String,
    ) -> Self {
        #[cfg(not(feature = "native-auth-flow"))]
        let esi = EsiBuilder::new()
            .user_agent(useragent)
            .client_id(client_id)
            .client_secret(_client_secret)
            .callback_url(callback_url)
            .scope(scope.join(" ").as_str())
            .build()
            .unwrap();

        #[cfg(feature = "native-auth-flow")]
        let esi = EsiBuilder::new()
            .user_agent(useragent)
            .client_id(client_id)
            .callback_url(callback_url)
            .enable_application_authentication(true)
            .scope(scope.join(" ").as_str())
            .build()
            .unwrap();

        let mut obj = EsiManager {
            esi,
            characters: Vec::new(),
            path: database_path,
            active_character: None,
        };

        // Path needs to be checked before invoking rusqlite to be effective
        let temp_path = Path::new(&obj.path);
        if !temp_path.exists() || !temp_path.is_file() {
            // TODO: migration database schema goes here
            let conn = obj.get_standart_connection();
            let _ = PlayerDatabase::create_database(&conn.unwrap());
            let _ = PlayerDatabase::migrate_database();
        } else {
            let conn = obj.get_standart_connection();
            // cargar jugadores existentes
            if let Ok(chars) = PlayerDatabase::select_characters(&conn.unwrap(), vec![]) {
                obj.characters = chars;
            }
        }

        obj
    }

    pub async fn get_location(&self, player_id: i32) -> Result<i32, Error> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_get_location");

        if let Ok(location) = self.esi.group_location().get_location(player_id).await {
            let player_location = location.solar_system_id;
            Ok(player_location)
        } else {
            Ok(0)
        }
    }

    #[tokio::main]
    pub async fn get_player_photo(url: &str) -> Result<Option<Vec<u8>>, String> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_get_player_photo");

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let uri = match url.parse::<hyper::Uri>() {
            Ok(parsed_uri) => parsed_uri,
            Err(t_error) => return Err(t_error.to_string() + " > " + url),
        };
        let mut resp;
        match client.get(uri).await {
            Ok(body) => resp = body,
            Err(t_error) => return Err(t_error.to_string()),
        }
        // And now...
        let mut photo = vec![];
        while let Some(Ok(chunk)) = resp.body_mut().data().await {
            photo.extend_from_slice(&chunk);
        }
        Ok(Some(photo))
    }

    pub async fn auth_user(
        &mut self,
        _auth_info: AuthenticationInformation,
        oauth_data: (String, String),
    ) -> Result<Option<Character>, Box<dyn std::error::Error + Send + Sync>> {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("esi_auth_user");

        #[cfg(not(feature = "native-auth-flow"))]
        let verifier = None;

        #[cfg(feature = "native-auth-flow")]
        let verifier = _auth_info.pkce_verifier;

        let claims_option = self
            .esi
            .authenticate(oauth_data.0.as_str(), verifier)
            .await?;
        if let Some(claims) = claims_option {
            
            let mut player = Character::new();
            //let data = claims.unwrap();
            //character name
            player.name = claims.name;
            //character id
            let split: Vec<&str> = claims.sub.split(':').collect();
            player.id = split[2].parse::<i32>().unwrap();
            if player.auth.is_some() {
                player.auth.as_mut().unwrap().token = self.esi.access_token.as_ref().unwrap().to_string();
                player.auth.as_mut().unwrap().refresh_token = self.esi.refresh_token.as_ref().unwrap().to_string();
                //expiration Date
                let expiration = Utc::now().checked_add_signed(TimeDelta::new(self.esi.access_token.as_ref().unwrap().parse::<i64>()?,0).unwrap());
                /*let expiration: DateTime<Utc> =
                    DateTime::parse_from_str(self.esi.access_token.as_ref().unwrap(), "%s")
                        .unwrap()
                        .into();*/
                player.auth.as_mut().unwrap().expiration = expiration;
                self.esi.update_spec().await?;

                let public_info = self
                    .esi
                    .group_character()
                    .get_public_info(player.id)
                    .await?;
                let corp_info = self
                    .esi
                    .group_corporation()
                    .get_public_info(public_info.corporation_id)
                    .await?;
                let corp = Corporation {
                    id: public_info.corporation_id,
                    name: corp_info.name,
                };
                player.corp = Some(corp);
                if let Some(ally_id) = public_info.alliance_id {
                    let ally_info = self.esi.group_alliance().get_info(ally_id).await?;
                    let ally = Alliance {
                        id: ally_id,
                        name: ally_info.name,
                    };
                    player.alliance = Some(ally);
                }
                let player_portraits = self.esi.group_character().get_portrait(player.id).await?;
                player.photo = Some(player_portraits.px128x128.unwrap());
                let player_location = self.esi.group_location().get_location(player.id).await?;
                player.location = player_location.solar_system_id;
                /*if let Some(photo_vec) = self.get_portrait_data(&player_portraits.px64x64.unwrap()).await?{
                        player.photo = Some(photo_vec);
                }*/
            }
            self.write_character(&player)?;
            Ok(Some(player))
        } else {
            Ok(None)
        }
    }
}
