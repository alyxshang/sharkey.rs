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

pub async fn cli() -> Result<String, SharkeyErr>{
    let result: String;
    let mut sharkey: App = App::new(
        "Sharkey",
        "0.2.0",
        "Alyx Shang"
    );
    sharkey.add_arg("react", "react to a note", &false);
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
    }

    // Unfollowing a user.
    else if sharkey.arg_was_used("dflow") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("namei") 
    {
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
    }

    // Deleting a note.
    else if sharkey.arg_was_used("rpost") &&
        sharkey.add_arg("tauth") &&
        sharkey.add_arg("apiad") &&
        sharkey.add_arg("inadd") &&
        sharkey.add_arg("namei") 
    {
    }

    // Reacting to a note.
    else if sharkey.arg_was_used("react") &&
        sharkey.arg_was_used("liken") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("namei") &&
        sharkey.arg_was_used("conte")
    {
    }

    // Deleting the reaction to a note.
    else if sharkey.arg_was_used("react") &&
        sharkey.arg_was_used("ulike") &&
        sharkey.arg_was_used("tauth") &&
        sharkey.arg_was_used("apiad") &&
        sharkey.arg_was_used("inadd") &&
        sharkey.arg_was_used("namei") &&
        sharkey.arg_was_used("conte")
    {
    }


    else {
        result = sharkey.help_info();
    }

    Ok(result)
}
