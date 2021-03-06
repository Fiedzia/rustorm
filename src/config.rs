use url::{Url, Host, SchemeData};



#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct DbConfig{
    /// postgres, sqlite, mysql
    /// some fields are optional since sqlite is not applicable for those
    pub platform: String,
    pub username: Option<String>,
    pub password: Option<String>,
    /// localhost
    pub host: Option<Host>,
    /// 5432
    pub port: Option<u16>,
    pub database: String,
}

impl DbConfig{
    
    /// TODO: get rid of the hacky way parsing database url
    /// https://github.com/servo/rust-url/issues/40
    pub fn from_url(url: &str)->Self{
        let parsed = Url::parse(url);
        match parsed {
            Ok(parsed) => {
                let non_relative = match parsed.scheme_data{
                        SchemeData::NonRelative(ref x) =>{
                            x
                        },
                        SchemeData::Relative(ref x)=> {
                            panic!("Expecting a NonRelative SchemeData {}",x)
                        }
                    };
                let scheme: &str = &parsed.scheme;
                // FIXME: This is a hacky way to parse database url, using servo/url parser
                let https_url = format!("https:{}", non_relative);
                let reparse = Url::parse(&https_url);
                
                let reparse_relative = match reparse {
                    Ok(reparse) =>{
                        match reparse.scheme_data{
                            SchemeData::Relative(ref relative) =>{
                                relative.clone()
                            },
                            SchemeData::NonRelative(ref x)=> {
                                panic!("Expecting a Relative SchemeData {}",x)
                            },
                        }
                    },
                    Err(e) => {
                        match url{
                            "sqlite:///:memory:" => {//special case for sqlite
                            return DbConfig{
                                    platform: scheme.to_string(),
                                    username: None,
                                    password: None,
                                    host: None,
                                    port: None,
                                    database: ":memory:".to_string()
                                    }    
                                },
                            _ =>panic!("error parsing https url:{}",e)
                        }
                    }
                };
                
               
            match scheme{
                "sqlite" => { // handle sqlite parsing such as the memory and the host be the database
                    DbConfig{
                        platform: scheme.to_string(),
                        username: None,
                        password: None,
                        host: None,
                        port: None,
                        database: {
                            match reparse_relative.host{
                                Host::Domain(ref db) => db.to_string(),
                                _ => panic!("only domains are handled"),
                            }
                        }    
                    }
                },
                _ =>
                    DbConfig{
                        platform: scheme.to_string(),
                        username: Some(reparse_relative.username.clone()),
                        password: reparse_relative.password.clone(),
                        host: Some(reparse_relative.host.clone()),
                        port: reparse_relative.port,
                        database: {
                            assert!(reparse_relative.path.len() == 1, "There should only be 1 path");
                            reparse_relative.path[0].to_string()
                        }    
                    }
                }
            }
            Err(e) => {
                println!("Error parsing url: {}", url);
                panic!("{}", e);
            }
        }
        
    }
    
    pub fn get_url(&self)->String{
        let mut url = String::new();
        url.push_str(&self.platform.to_string());
        url.push_str("://");
        if self.username.is_some(){
            url.push_str(self.username.as_ref().unwrap());
        }
        if self.password.is_some(){
            url.push_str(":");
            url.push_str(self.password.as_ref().unwrap());
        }
        
        if self.host.is_some(){
            url.push_str("@");
            url.push_str(&self.host.as_ref().unwrap().serialize());
        }
        
        if self.port.is_some(){
            url.push_str(":");
            url.push_str(&format!("{}", self.port.as_ref().unwrap()));
        }
        url.push_str("/");
        url.push_str(&self.database);
        url
    }
}

#[test]
fn test_config_url(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let config = DbConfig{
        platform: "postgres".to_string(),
        username: Some("postgres".to_string()),
        password: Some("p0stgr3s".to_string()), 
        host: Some(Host::Domain("localhost".to_string())),
        port: None,
        database: "bazaar_v6".to_string(),
    };
    
    assert_eq!(config.get_url(), url.to_string());
}

#[test]
fn test_config_from_url(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let config = DbConfig::from_url(url);
    assert_eq!(config.get_url(), url.to_string());
}


#[test]
fn test_config_url_with_port(){
    let url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v6";
    let config = DbConfig{
        platform: "postgres".to_string(),
        username: Some("postgres".to_string()),
        password: Some("p0stgr3s".to_string()), 
        host: Some(Host::Domain("localhost".to_string())),
        port: Some(5432),
        database: "bazaar_v6".to_string(),
    };
    
    assert_eq!(config.get_url(), url.to_string());
}

#[test]
fn test_config_sqlite_url_with_port(){
    let url = "sqlite:///bazaar_v6.db";
    let parsed_config = DbConfig::from_url(url);
    let expected_config = DbConfig{
        platform: "sqlite".to_string(),
        username: None,
        password: None, 
        host: None,
        port: None,
        database: "bazaar_v6.db".to_string()
    };
    println!("{:?}",parsed_config);
    assert_eq!(parsed_config, expected_config);
}

#[test]
fn sqlite_in_memory(){
    let url = "sqlite:///:memory:";
    let parsed_config = DbConfig::from_url(url);
    let expected_config = DbConfig{
        platform: "sqlite".to_string(),
        username: None,
        password: None, 
        host: None,
        port: None,
        database: ":memory:".to_string(),
    };
    println!("{:?}",parsed_config);
    assert_eq!(parsed_config, expected_config);
}
