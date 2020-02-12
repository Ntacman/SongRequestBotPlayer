#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate tera;

use std::env;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use rocket::Request;
use std::collections::VecDeque;
use std::sync::{Mutex, Arc};
use tera::Context;

#[derive(Serialize, Deserialize)]
struct PlaylistItem {
  url: String,
  name: String,
}

lazy_static! {
  static ref PLAYLISTMUTEX: Arc<Mutex<VecDeque<PlaylistItem>>> = Arc::new(Mutex::new(VecDeque::new()));
}

#[get("/playlist")]
fn playlist_info() -> JsonValue {
  json!(
    {"items": *PLAYLISTMUTEX.clone().lock().unwrap()}
  )
}

#[get("/get_next_song")]
fn get_next_song() -> JsonValue {
  let song = PLAYLISTMUTEX.clone().lock().unwrap().pop_front();
  json!(song)
}

#[get("/")]
fn index() -> Template {
  let song = PLAYLISTMUTEX.clone().lock().unwrap().pop_front();
  let context = song.unwrap_or(PlaylistItem{url: "empty".to_owned(), name: "empty".to_owned()});
  Template::render("index", &context)
}

#[put("/add", data = "<item>")]
fn add_song(item: Json<PlaylistItem>) -> JsonValue {
  PLAYLISTMUTEX.clone().lock().unwrap().push_back(item.0);
  json!({"status": "ok", "items": *PLAYLISTMUTEX.clone().lock().unwrap()})
}

fn main() {
  rocket::ignite()
    .mount("/api/", routes![playlist_info, add_song, get_next_song])
    .mount("/", routes![index])
    .mount("/static", StaticFiles::from("static"))
    .attach(Template::fairing())
    .launch();
}
