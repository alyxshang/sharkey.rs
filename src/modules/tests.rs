/*
Sharkey.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "tokio" namespace
/// to use the "test"
/// macro.
use tokio;

/// The function to test
/// the "fetch_json" function.
#[tokio::test]
pub async fn test_fetch_json(){
    let mut payload: std::collections::HashMap<String,String> = std::collections::HashMap::new();
    payload.insert("lib_name".to_string(), "sharkey.rs".to_string());
    match super::network::fetch_json(
        &super::enums::HTTPMethods::GET, 
        &payload, 
        "https://httpbin.org/json"
    ).await {
        Ok(resp) => {
            match resp.body {
                Some(val) => {
                    assert_eq!(val.is_empty(), false);
                },
                None => {
                    eprintln!("Could not fetch response!");
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}


/// The function to test
/// the "like_note_for_user" and the
/// "unlike_note_for_user" function.
#[tokio::test]
pub async fn test_note_reaction(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {
            match super::actions::like_note_for_user(
                "/api", 
                "https://blahaj.zone", 
                &value.clone(),
                "9zpo9el4sh0901es",
                "like"
            ).await {
                Ok(res) => {
                    match res.body {
                        Some(val) => {
                            println!("{}", val);
                        },
                        None => {
                            assert_eq!(false, false);
                            match super::actions::unlike_note_for_user(
                                "/api", 
                                "https://blahaj.zone", 
                                &value.clone(),
                                "9zpo9el4sh0901es",
                                "like"
                            ).await {
                                Ok(res) => {
                                    match res.body {
                                        Some(val) => println!("{}", val),
                                        None => {
                                            assert_eq!(false, false);
                                        }
                                    }
                                },
                                Err(x) => {
                                    eprintln!("{}", x);
                                }
                            }
                        }
                    }
                },
                Err(x) => {
                    eprintln!("{}", x);
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }    
}

/// The function to test
/// the "follow_user" and the 
/// "unfollow_user" function.
#[tokio::test]
pub async fn test_follow_action_user(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {            
            match super::actions::follow_user(
                "/api", 
                "https://blahaj.zone", 
                &value.clone(),
                "9upmnr8igmxe01k3"
            ).await {
                Ok(res) => {
                    assert_eq!(res.username, "frisaf");
                    match super::actions::unfollow_user(
                        "/api", 
                        "https://blahaj.zone", 
                        &value.clone(),
                        "9upmnr8igmxe01k3"

                    ).await {
                        Ok(res) => {
                            assert_eq!(res.username, "frisaf");
                        },
                        Err(x) => {
                            eprintln!("{}", x);
                        }
                    }
                },
                Err(y) => {
                    eprintln!("{}", y);
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }    
}

/// The function to test
/// the "create_note_for_user" function.
#[tokio::test]
pub async fn test_create_note_for_user(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {
            
            match super::actions::create_note_for_user(
                "/api", 
                "https://blahaj.zone", 
                &value,
                &super::enums::NoteVisibility::Public,
                &Some(super::enums::ReactionAcceptance::LikeOnly),
                "This note was posted from the \"Sharkey.rs\" test runner."
            ).await {
                Ok(res) => {
                    assert_eq!(&res.created_note.user.username, &"alyxshang".to_string());
                },
                Err(x) => {
                    eprintln!("{}", x);
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }    
}

/// The function to test
/// the "delete_note_for_user" function.
#[tokio::test]
pub async fn test_delete_note_for_user(){
    match std::env::var("BLAHAJ_API_TOKEN"){
        Ok(value) => {
            match super::actions::create_note_for_user(
                "/api", 
                "https://blahaj.zone", 
                &value,
                &super::enums::NoteVisibility::Public,
                &Some(super::enums::ReactionAcceptance::LikeOnly),
                "This note only exists to be deleted."
            ).await {
                Ok(res) => {
                    let note_to_be_deleted = &res.created_note.id;
                    match super::actions::delete_note_for_user(
                        "/api", 
                        "https://blahaj.zone", 
                        &value.clone(),
                        &note_to_be_deleted
                    ).await {
                        Ok(resp) => {
                            assert_eq!(resp.body, None);
                        },
                        Err(y) => eprintln!("{}", y)
                    };
                },
                Err(x) => {
                    eprintln!("{}", x);
                }
            }
        },
        Err(e) => {
           eprintln!("{}", e);
        }
    }    
}
