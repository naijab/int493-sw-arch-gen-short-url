use rand::Rng;
use rocket_contrib::json::{JsonValue};
use postgres::{Client, NoTls, Row};

struct ShortUrl {
    id: i32,
    short_url: String,
    full_url: String,
    user_id: i32,
}

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

    let short_url = url_shorten();
    let mut is_short_url_valid = false;

    while !is_short_url_valid {
        is_short_url_valid = check_short_url_is_valid(&short_url);
    }

    if is_short_url_valid {
        let _ = save_short_url(&short_url, &full_url);
    }

    return json!({
        "full_url": full_url,
        "short_url": format!("http://localhost:8000/{}", short_url),
    })
}

fn url_shorten() -> String {
    let str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let char_vec: Vec<char> = str.chars().collect();
    
    let mut rng = rand::thread_rng();
    
    let ch1 = rng.gen_range(0..61);
    let ch2 = rng.gen_range(0..61);
    let ch3 = rng.gen_range(0..61);
    let ch4 = rng.gen_range(0..61);
    let ch5 = rng.gen_range(0..61);
    let ch6 = rng.gen_range(0..61);

    format!("{}{}{}{}{}{}",
            char_vec[ch1],
            char_vec[ch2],
            char_vec[ch3],
            char_vec[ch4],
            char_vec[ch5],
            char_vec[ch6])
}

fn check_short_url_is_valid(short_url: &String) -> bool {
    // FIXME: Cannot Connect DB
    let mut db_client = Client::connect("host=localhost port=26257 user=root dbname=links_db", NoTls).unwrap();

    let result = db_client.query_one("SELECT * id FROM short_urls WHERE short_url=$1", &[&short_url]).unwrap();

    result.is_empty()
}

fn save_short_url(short_url: &String, full_url: &String) -> bool {
    // FIXME: Cannot Connect DB
    let mut db_client = Client::connect("host=roach_node_1 port=6257 user=root dbname=links_db", NoTls).unwrap();

    // TODO: Use user id in future for who create links;
    let _user_id = 1;

    let result = db_client.execute(
        "INSERT INTO short_urls (short_url, full_url, user_id) VALUES ($1, $2, $3)",
        &[&short_url.to_string(), &full_url.to_string(), &_user_id.to_string()],
    ).unwrap();

    result > 0
}