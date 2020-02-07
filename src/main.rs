#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::env;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

//#[get("/")]
//fn index() -> &'static str {
//  "Hello, world"
//}

#[derive(Serialize, Deserialize)]
struct PlaylistItem {
  url: String,
  name: String,
  index: u16,
}

static mut playlist: Vec<PlaylistItem> = Vec::new();

#[get("/playlist")]
fn playlist_info() -> JsonValue {
  json!(
    {
      "items": unsafe { &playlist }
    }
  )
}

#[put("/add", data = "<item>")]
fn add_song(item: Json<PlaylistItem>) -> JsonValue {
  unsafe { &playlist.push(item.0); }
  json!({"status": "ok"})
}

fn main() {
  let dir = ::std::env::current_dir().unwrap();
  let path:&str = &dir.to_str().unwrap().to_string();
  println!("Static File Path is : {}/static", path);

  unsafe{
    &playlist.push(PlaylistItem{url: "test".to_owned(), name: "test".to_owned(), index: 0});
    &playlist.push(PlaylistItem{url: "test1".to_owned(), name: "test1".to_owned(), index: 1});
    &playlist.push(PlaylistItem{url: "test2".to_owned(), name: "test2".to_owned(), index: 2});
  }
  
  rocket::ignite()
    .mount("/api/", routes![playlist_info, add_song])
    .mount("/", StaticFiles::from(format!("{}{}", path, "/static")))
    .launch();
}
