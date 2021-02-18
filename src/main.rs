#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rusqlite::{Connection, Statement};
use serde::{Serialize , Deserialize};
use urlshortener::{providers::Provider, client::UrlShortener};
use rocket::response::{Redirect , Responder, Response};
use rusqlite::types::FromSql;
use rocket::request::{Request};
use rocket::http::{Status};
use rand::Rng;
use rocket_contrib::{
    json,
    json::{Json, JsonValue,JsonError}
    };


#[derive(Serialize)]
struct UrlList {
    url_items: Vec<KeptUrl>,
}

#[derive(Serialize , Debug)]
struct KeptUrl {
    id: i64,
    url: String,
    link: String,
}

//for response status msg
#[derive(Serialize)]
struct StatusMessage {
    message: String,
}


#[derive(Serialize)]
struct FullURLList {
    full_url_items: Vec<FullURL>,
}

#[derive(Serialize)]
struct FullURL {
    url: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/l/<link>")]
fn link_url(link: String) -> Redirect{
    
    //println!("link: {}",link);
    let redirect_url = "https://github.com/";


    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection
        ,Err(_) =>  return Redirect::to(format!("{}",redirect_url))
    };
    
    let mut statement = match db_connection.prepare("select url from url_list where link like 'vzSFzF' ;"){
        Ok(statement) => statement,
        Err(_) => return Redirect::to(format!("{}",redirect_url))
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(FullURL{
            url: row.get(0)?,
        })
    });
    
    let mut emp1 = FullURL{
        url:String::from("https://translate.google.com/"),
     };

    return Redirect::to(format!("{}",emp1.url));
    
}


#[get("/kepturl")]
fn fetch_all_url_itmes() -> Result<Json<UrlList>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare("select id,url,link from url_list;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(KeptUrl {
            id: row.get(0)?,
            url: row.get(1)?,
            link: row.get(2)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(url_items) => Ok(Json(UrlList { url_items })),
                Err(_) => Err("Could not collect url_items".into()),
            }
        }
        Err(_) => Err("Failed to fetch url items".into()),
    }
}

// Using Provider
/*
#[post("/kepturl", format = "json", data = "<item>")]
fn add_url(item: Json<String>) -> Result<Json<StatusMessage>, String> {

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let us = UrlShortener::new().unwrap();
    let short_url = us.generate( item.as_str() , &Provider::IsGd);


    let mut statement =
        match db_connection.prepare("insert into url_list (id,url,link) values (null,$1,$2);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };
    let results = statement.execute(&[&item.0 , &short_url.unwrap()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(_) => Err("Failed to insert todo item".into()),
    }
}
*/
#[post("/kepturl", format = "json", data = "<item>")]
fn add_url(item: Json<String>) -> Result<Json<StatusMessage>, String> {

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

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

    let mut statement =
        match db_connection.prepare("insert into url_list (id,url,link) values (null,$1,$2);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };
    let results = statement.execute(&[&item.0 , &short_url]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(_) => Err("Failed to insert todo item".into()),
    }
}


#[delete("/kepturl/<id>")]
fn remove_url_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare("delete from url_list where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted!", rows_affected),
        })),
        Err(_) => Err("Failed to delete url item".into()),
    }
}

fn main() {
    {
        let db_connection = Connection::open("data.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists url_list (
                    id integer primary key,
                    url varchar(64) NOT NULL UNIQUE,
                    link varchar(64) NOT NULL UNIQUE
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();
    }

    rocket::ignite().mount( "/",routes![index, fetch_all_url_itmes, add_url, remove_url_item , link_url], ).launch();
}


/*
#[post("/shortUrl")]
fn short_url() -> &'static str{
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
    
    return short_url.as_str()
}
*/