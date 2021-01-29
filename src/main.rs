#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct TextRes {
    text: String
}

#[post("/speak", format="json", data="<value>")]
fn speak(value: Json<TextRes>) -> String {
    std::process::Command::new("say").args(&[format!("\"{}\"", &value.text.replace("\"", ""))]).output().expect("erroring");
    format!("hello")
}

fn main() {
    println!("hello");
    rocket::ignite().mount("/",routes![speak]).launch();
}
