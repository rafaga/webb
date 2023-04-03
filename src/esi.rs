use rfesi::prelude::*;
use futures::executor::block_on;
use tokio::time::{Instant, timeout_at};
use tokio::time::Duration;
use std::net::SocketAddr;
use hyper::Server;
use crate::auth_service::MakeSvc;
use crate::database::Character;


pub struct EsiManager{
    pub esi: Esi,
    pub characters: Vec<Character>,
}

impl EsiManager {

    pub fn new(useragent: &str, client_id: &str, client_secret: &str, callback_url: &str) -> Self {
        
        let scope = vec!["publicData","esi-alliances.read_contacts.v1","esi-characters.read_chat_channels.v1",
            "esi-characters.read_contacts.v1","esi-characters.read_fatigue.v1","esi-characters.read_standings.v1",
            "esi-clones.read_clones.v1","esi-clones.read_implants.v1","esi-corporations.read_contacts.v1","esi-corporations.read_standings.v1",
            "esi-corporations.read_starbases.v1","esi-corporations.read_structures.v1","esi-location.read_location.v1",
            "esi-location.read_online.v1","esi-location.read_ship_type.v1","esi-search.search_structures.v1",
            "esi-skills.read_skills.v1","esi-ui.write_waypoint.v1","esi-universe.read_structures.v1"];

        let esi = EsiBuilder::new()
            .user_agent(useragent)
            .client_id(client_id)
            .client_secret(client_secret)
            .callback_url(callback_url)
            .scope(scope.join(" ").as_str())
            .build().unwrap();

        EsiManager {
            esi,
            characters: Vec::new(),
        }
    }

    #[tokio::main]
    pub async fn auth_user(&mut self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = ([127, 0, 0, 1], 56123).into();
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
            return Ok(false);
        };

        block_on(async {
            let claims = self.esi.authenticate(result.0.as_str()).await;
            let mut char = Character::new();
            let data = claims.unwrap().unwrap();
            char.name = data.name;
            self.characters.push(char);
        });

        Ok(true)
    }
}