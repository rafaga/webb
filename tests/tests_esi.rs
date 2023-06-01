
#[cfg(test)]
mod test_esi {
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_esi_get_public_data() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];
        let path_str = Some("tests/tests2.db");
        let path = Path::new(path_str.unwrap());
        let mut vec = Vec::new();
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path);
        }

        let mut esimon = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, path_str); 
        let (url,_rand) = esimon.esi.get_authorize_url().unwrap();
        
        match open::that(&url){
            Ok(()) => {
                match esimon.auth_user(4500){
                    Ok(Some(player)) => {
                        vec.push(player);
                        assert_ne!(vec[0].name,"");
                    },
                    Ok(None) => {
                        panic!("No user has been authenticated");
                    },
                    Err(esi_error) => {
                        panic!("An error occurred has ocurred: {}", esi_error);
                    }
                }
            },
            Err(err) => panic!("An error occurred when opening '{}': {}", url, err),
        }
    }

    

}