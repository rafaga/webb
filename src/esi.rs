pub mod data;

use crate::esi::data::Data;
use rfesi::prelude::*;

pub struct EsiManager{
    data: Data,
    pub esi: Esi,
}

impl EsiManager {

    pub fn new() -> Self {
        let data = Data::new();

        let esi = EsiBuilder::new()
            .user_agent(&data.user_agent)
            .client_id(&data.client_id)
            .client_secret(&data.secret_key)
            .callback_url(&data.callback_url)
            .build().unwrap();

        EsiManager {
            data,
            esi,
        }
    }
}