use rocket_contrib::json::{JsonValue};

#[get("/url_shortener")]
pub fn check_url() -> JsonValue {
    json!({
        "status": "Hello World!",
    })
}