use rocket_contrib::json::{JsonValue};
use rand::Rng;

#[get("/<short_url>")]
pub fn get_full_url(short_url: String) -> JsonValue {
    if short_url.trim().is_empty() {
        return json!({
            "error": "request invalid, please put short url slug",
        })
    }
    return json!({
        "short_url": format!("http://localhost:8000/{}", short_url),
    })
}

#[get("/url_shortener?<full_url>")]
pub fn get_short_url(full_url: String) -> JsonValue {
    if full_url.trim().is_empty() {
        return json!({
            "error": "request invalid, please put full url query",
        })
    }

    let url = shorten();
    return json!({
        "full_url": full_url,
        "short_url": format!("http://localhost:8000/{}", url),
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