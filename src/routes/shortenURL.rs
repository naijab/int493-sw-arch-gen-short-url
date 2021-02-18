fn main() {
    println!("ShortenURL(Non lib) : {}", get_short_url());
    println!("ShortenURL(lib)     : ");
    get_short_url_lib();
}

fn get_short_url() -> String {
    //Try to make
    use rand::Rng;
    let alphanumeric_keys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    let _chars: Vec<char> = alphanumeric_keys.chars().collect();
    let mut short_url = String::new();
    let mut rng = rand::thread_rng();
    
    let mut n = 0;
    
    //Change condition for length of url here.
    while n<6 {
        short_url.push( alphanumeric_keys.chars().nth(rng.gen_range(0..61)).unwrap() as char );
        n+=1;
    }
    
    return short_url

}

fn get_short_url_lib() {
    //Read Docs
    //https://docs.rs/urlshortener/3.0.0/urlshortener/
    //An easy library for retrieving short urls.

    use urlshortener::{providers::Provider, client::UrlShortener};

    let us = UrlShortener::new().unwrap();
    //let short_url = us.generate("https://github.com/patanasak", &Provider::IsGd);
    print!("{:?}",us.generate("https://github.com/patanasak" , &Provider::IsGd));
    //return short_url.is_ok().to_string();

}
