#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use std::env;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;
use std::collections::VecDeque;
use std::sync::{Mutex, Arc};

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

lazy_static! {
  static ref PLAYLISTMUTEX: Arc<Mutex<VecDeque<PlaylistItem>>> = Arc::new(Mutex::new(VecDeque::new()));
}

#[get("/playlist")]
fn playlist_info() -> JsonValue {
  json!({"items": "test"})
}

#[put("/add", data = "<item>")]
fn add_song(item: Json<PlaylistItem>) -> JsonValue {
  PLAYLISTMUTEX.clone().lock().unwrap().push_back(item.0); 
  json!({"status": "ok"})
}

fn main() {
  let dir = ::std::env::current_dir().unwrap();
  let path:&str = &dir.to_str().unwrap().to_string();
  println!("Static File Path is : {}/static", path);

  unsafe{
    PLAYLISTMUTEX.clone().lock().unwrap().push_back(PlaylistItem{url: "test".to_owned(), name: "test".to_owned(), index: 0});
    PLAYLISTMUTEX.clone().lock().unwrap().push_back(PlaylistItem{url: "test1".to_owned(), name: "test1".to_owned(), index: 1});
    PLAYLISTMUTEX.clone().lock().unwrap().push_back(PlaylistItem{url: "test2".to_owned(), name: "test2".to_owned(), index: 2});
  }
  
  for x in PLAYLISTMUTEX.clone().lock().unwrap().iter() {
    println!("{}{}{}", x.url, x.name, x.index);
  }
  rocket::ignite()
    .mount("/api/", routes![playlist_info, add_song])
    .mount("/", StaticFiles::from(format!("{}{}", path, "/static")))
    .launch();
}
