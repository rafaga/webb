pub mod data;

use crate::esi::data::Data;
use rfesi::prelude::*;

#[derive(Clone)]
pub struct EsiManager{
    pub esi: Esi,
}

impl EsiManager {

    pub fn new(data: Data) -> Self {
        
        let scope = vec!["publicData","esi-alliances.read_contacts.v1","esi-characters.read_chat_channels.v1",
            "esi-characters.read_contacts.v1","esi-characters.read_fatigue.v1","esi-characters.read_standings.v1",
            "esi-clones.read_clones.v1","esi-clones.read_implants.v1","esi-corporations.read_contacts.v1","esi-corporations.read_standings.v1",
            "esi-corporations.read_starbases.v1","esi-corporations.read_structures.v1","esi-location.read_location.v1",
            "esi-location.read_online.v1","esi-location.read_ship_type.v1","esi-search.search_structures.v1",
            "esi-skills.read_skills.v1","esi-ui.write_waypoint.v1","esi-universe.read_structures.v1"];

        let esi = EsiBuilder::new()
            .user_agent(&data.user_agent)
            .client_id(&data.client_id)
            .client_secret(&data.secret_key)
            .callback_url(&data.callback_url)
            .scope(scope.join(" ").as_str())
            .build().unwrap();

        EsiManager {
            esi,
        }
    }
}