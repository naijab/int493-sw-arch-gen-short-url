use rocket_contrib::json::{JsonValue};
use rand::Rng;

#[get("/url_shortener")]
pub fn get_url() -> JsonValue {
    let url = shorten();
    return json!({
        "url": format!("http://localhost:8000/{}", url),
    })
}

fn shorten() -> String {
    let str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let char_vec: Vec<char> = str.chars().collect();
    
    let mut rng = rand::thread_rng();
    
    let ch1 = rng.gen_range(0..61);
    let ch2 = rng.gen_range(0..61);
    let ch3 = rng.gen_range(0..61);
    let ch4 = rng.gen_range(0..61);
    let ch5 = rng.gen_range(0..61);
    let ch6 = rng.gen_range(0..61);

    return format!("{}{}{}{}{}{}", 
                    char_vec[ch1], 
                    char_vec[ch2], 
                    char_vec[ch3], 
                    char_vec[ch4], 
                    char_vec[ch5], 
                    char_vec[ch6]);
}