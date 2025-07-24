mod china_festival;
mod three_aztec_temples;


pub async fn execute(a_game_name: String, a_location: String) {
    match a_game_name.as_str() {
        "china_festival" => {china_festival::execute(a_game_name, a_location).await}
        //"coin_lamp" => {china_festival::execute(a_game_name, a_location).await}
        "three_aztec_temples" => {three_aztec_temples::execute(a_game_name, a_location).await}
        _ => {eprintln!("\r\tGame not implement");}
    }
}