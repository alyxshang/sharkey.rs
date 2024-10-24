/*
Sharkey.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "App"
/// structure from the "cliply"
/// crate to make a new CLI app.
use cliply::App;

/// Importing the "SharkeyErr"
/// structure to catch and handle
/// errors.
use super::error::SharkeyErr;

/// Importing the function to
/// follow a user.
use super::actions::follow_user;

/// Importing this enum to set the scope
/// of viewers for a note.
use super::enums::NoteVisibility;

/// Importing the function to
/// unfollow a user.
use super::actions::unfollow_user;


/// Importing this enum to set
/// which type of reactions can be
/// sent to a note.
use super::enums::ReactionAcceptance;

/// Importing the function to like a note
/// for a user.
use super::actions::like_note_for_user;

/// Importing the function to create a note
/// for a user.
use super::actions::create_note_for_user;

/// Importing the function to unlike a note
/// for a user.
use super::actions::unlike_note_for_user;

/// Importing the function to delete a note
/// for a user.
use super::actions::delete_note_for_user;

pub async fn cli() -> Result<String, SharkeyErr>{
    let result: String;
    let mut sharkey: App = App::new(
        "Sharkey",
        "0.2.0",
        "Alyx Shang"
    );
    sharkey.add_arg("postn", "perform a posting action", &true);
    sharkey.add_arg("rpost", "remove a post", &false);
    sharkey.add_arg("liken", "send a reaction to a note", &false);
    sharkey.add_arg("ulike", "unsend a reaction to a note", &false);
    sharkey.add_arg("tauth", "the token to use for authentication", &true);
    sharkey.add_arg("apiad", "the server route for API requests", &true);
    sharkey.add_arg("inadd", "the address of the instance", &true);
    sharkey.add_arg("conte", "the content of the entity being sent", &true);
    sharkey.add_arg("namei", "the ID of the entity", &true);
    sharkey.add_arg("visie", "the visibility of the entity", &true);
    sharkey.add_arg("etype", "the type of reaction on an entity", &true);
    sharkey.add_arg("mflow", "follow a user", &false);
    sharkey.add_arg("dflow", "unfollow a user", &false);

    if sharkey.version_is(){
        result = sharkey.version_info();
    }
    else if sharkey.help_is(){
        result = sharkey.help_info();
    }

    // Following a user.
    else if sharkey.arg_was_used("mflow") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("namei")
    {
        let token: String = sharkey.get_arg_data("tauth"){
            Ok(token) => token,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let api_route: String = sharkey.get_arg_data("apiad"){
            Ok(api_route) => api_route,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let instance_addr: String = sharkey.get_arg_data("inadd"){
            Ok(instance_addr) => instance_addr,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let name_id: String = sharkey.get_arg_data("namei"){
            Ok(name_id) => name_id,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let res: String = match follow_user(
            &api_route,
            &instance_addr,
            &token,
            &name_id
        ){
            Ok(feedback) => format!("User \"{}\" has been followed!", feedback.username),
            Err(e) => return Err::<String, SharkeyErr>(&e.to_string()
        };
        result = res;
    }

    // Unfollowing a user.
    else if sharkey.arg_was_used("dflow") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("namei") 
    {
        let token: String = sharkey.get_arg_data("tauth"){
            Ok(token) => token,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let api_route: String = sharkey.get_arg_data("apiad"){
            Ok(api_route) => api_route,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let instance_addr: String = sharkey.get_arg_data("inadd"){
            Ok(instance_addr) => instance_addr,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let name_id: String = sharkey.get_arg_data("namei"){
            Ok(name_id) => name_id,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let res: String = match unfollow_user(
        ){
            &api_route,
            &instance_addr,
            &token,
            &name_id
        }{
            Ok(feedback) => format!("User \"{}\" has been unfollowed", feedback.username),
            Err(e) => return Err::<String, ShakreyErr>(&e.to_string())
        }
        result = res;
    }

    // Posting a note.
    else if sharkey.arg_was_used("postn") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("conte") &&
        sharkey.arg_was_used("visie") &&
        sharkey.arg_was_used("etype")
    {
        let token: String = sharkey.get_arg_data("tauth"){
            Ok(token) => token,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let api_route: String = sharkey.get_arg_data("apiad"){
            Ok(api_route) => api_route,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let instance_addr: String = sharkey.get_arg_data("inadd"){
            Ok(instance_addr) => instance_addr,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let entity_content: String = sharkey.get_arg_data("conte"){
            Ok(entity_content) => entity_content,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let visib: String = sharkey.get_arg_data("visie"){
            Ok(visib) => visib,
            Err(e) => return Err::<String, SharkeyErr>(&e.to_string())
        };
        let etype: String = sharkey.get_arg_data("etype"){
            Ok(etype) => etype,
            Err(e) => return Err::<String, SharkeyErr>(&e.to_string())
        };
        let visibility: NoteVisibility;
        match visib.as_str() {
            "home" => visibility = NoteVisibility::Home,
            "public" => visibility = NoteVisibility::Public,
            "followers" => visibility = NoteVisibility::Followers,
            _ => visibility = NoteVisbility::Home
        };
        let rec_type: ReactionAcceptance;
        match etype.as_str() {
            "LikeOnly" => rec_type = ReactionAcceptance::LikeOnly,
            "NSOnly" => rec_type = ReactionAcceptance::NonSensitiveOnly,
            "LOFRemote" => rec_type = ReactionAcceptance::LikeOnlyForRemote,
            _ => rec_type = ReactionAcceptance::LikeOnly
        }:
        let res: String = match create_note_for_user(
            &api_route,
            &instance_addr,
            &token,
            &visibility,
            &Some(rec_type)
        ){
            Ok(feedback) => format!("Note with the ID \"{}\" created.", feedback.created_note.id),
            Err(e) => return Err::<String, SharkeyErr>(&e.to_string())
        };
        result = res;

    }

    // Deleting a note.
    else if sharkey.arg_was_used("rpost") &&
        sharkey.add_arg("tauth") &&
        sharkey.add_arg("apiad") &&
        sharkey.add_arg("inadd") &&
        sharkey.add_arg("namei") 
    {
        let token: String = sharkey.get_arg_data("tauth"){
            Ok(token) => token,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let api_route: String = sharkey.get_arg_data("apiad"){
            Ok(api_route) => api_route,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let instance_addr: String = sharkey.get_arg_data("inadd"){
            Ok(instance_addr) => instance_addr,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let name_id: String = sharkey.get_arg_data("namei"){
            Ok(name_id) => name_id,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let res: String = match delete_note_for_user(
            &api_route,
            &instance_addr,
            &token,
            &name_id
        ){
            Ok(_feedback) => format!("Note with ID \"{}\" deleted.", &name_id),
            Err(e) => return Err::<String, SharkeyErr>(&e.to_string())
        };
        result = res;
    }

    // Reacting to a note.
    else if sharkey.arg_was_used("liken") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("namei") &&
        sharkey.arg_was_used("conte")
    {
        let token: String = sharkey.get_arg_data("tauth"){
            Ok(token) => token,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let api_route: String = sharkey.get_arg_data("apiad"){
            Ok(api_route) => api_route,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let instance_addr: String = sharkey.get_arg_data("inadd"){
            Ok(instance_addr) => instance_addr,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let name_id: String = sharkey.get_arg_data("namei"){
            Ok(name_id) => name_id,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let entity_content: String = sharkey.get_arg_data("conte"){
            Ok(entity_content) => entity_content,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let res: String = match like_note_for_user(
            &api_route,
            &instance_addr,
            &token,
            &name_id,
            &entity_content
        ){
            Ok(_feedback) => "React sent.".to_string(),
            Err(e) => return Err::<String, ShakreyErr>(&e.to_string())
        };
        result = res;
    }

    // Deleting the reaction to a note.
    else if sharkey.arg_was_used("ulike") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("namei") &&
        sharkey.arg_was_used("conte")
    {
        let token: String = sharkey.get_arg_data("tauth"){
            Ok(token) => token,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let api_route: String = sharkey.get_arg_data("apiad"){
            Ok(api_route) => api_route,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let instance_addr: String = sharkey.get_arg_data("inadd"){
            Ok(instance_addr) => instance_addr,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let name_id: String = sharkey.get_arg_data("namei"){
            Ok(name_id) => name_id,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let entity_content: String = sharkey.get_arg_data("conte"){
            Ok(entity_content) => entity_content,
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        let res: String = match ulike_note_for_user(
            &api_route,
            &instance_addr,
            &token,
            &name_id,
            &entity_content
        ){
            Ok(_feedback) => "Reaction deleted.".to_string(),
            Err(e) => return Err::<String,SharkeyErr>(&e.to_string())
        };
        result = res;
    }


    else {
        result = sharkey.help_info();
    }

    Ok(result)
}
