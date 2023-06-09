
pub struct Data{
    pub user_agent:String,
    pub client_id:String,
    pub secret_key: String,
    pub callback_url: String,
    pub authorize_url: String,
    pub random_state: String,
}

impl Data{
    pub fn new() -> Self {
        Data { 
            user_agent: String::new(), 
            client_id: String::new(), 
            secret_key: String::new(), 
            callback_url: String::new(),
            random_state: String::new(),
            authorize_url: String::new(),
        }
    }
}