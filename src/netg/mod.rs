mod jocker_twenty;


pub async fn execute(a_game_name: String, a_location: String) {
    match a_game_name.as_str() {
        "jocker_twenty" => {jocker_twenty::execute(a_game_name, a_location).await}
        _ => {eprintln!("\r\tGame not implement");}
    }
}