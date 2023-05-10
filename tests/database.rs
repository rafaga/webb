#[cfg(test)]
mod tests_database {
    use std::path::Path;
    use std::fs;

    use webb::objects::{Character, Corporation, Alliance};

    #[test]
    fn test_database_creation() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];

        let path = Path::new("tests/tests.db");
        if path.exists() && path.is_file() {
            let _ = fs::remove_file("tests/tests-create.db");
        }
        let _ = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, Some("tests/tests.db")); 
        assert!(path.exists());
        if path.exists() {
            let _ = fs::remove_file("tests/tests-create.db");
        }
    }

    #[test]
    fn test_web_auth() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];

        let mut esimon = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, None); 
        let (url,_rand) = esimon.esi.get_authorize_url().unwrap();
        match open::that(&url){
            Ok(()) => {
                if let Ok(Some(player)) = esimon.auth_user(4500){

                    assert_ne!(player.name,"");
                }
            },
            Err(err) => panic!("An error occurred when opening '{}': {}", url, err),
        }
    }

    #[test]
    #[should_panic]
    fn test_characters_no_dup() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];
        let path_str = Some("tests/tests1.db");
        let path = Path::new(path_str.unwrap());
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path);
        }
        let esimon = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, path_str);
        
        let mut vec = Vec::new();
        for _n in [0..2] {
            let mut char = Character::new();
            char.id = 1;
            char.name = String::from("test");
            char.corp = Some(Corporation{id:12, name:String::from("test")});
            char.alliance = Some(Alliance{id:2, name:String::from("test")});
            vec.push(char);
        }
        esimon.add_characters(vec);
    }

    #[test]
    fn test_add_character() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];
        let path_str = Some("tests/tests2.db");
        let path = Path::new(path_str.unwrap());
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path);
        }
        let esimon = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, path_str);
        let mut vec = Vec::new();
        let mut char = Character::new();
        char.id = 1;
        char.name = String::from("test");
        char.corp = Some(Corporation{id:12, name:String::from("test")});
        char.alliance = Some(Alliance{id:2, name:String::from("test")});
        vec.push(char);
        assert_eq!(esimon.add_characters(vec),Ok(true));
    }

    

}