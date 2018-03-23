#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::sync::RwLock;
use std::collections::HashMap;
use rocket::State;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8, state: State<RwLock<HashMap<String, u8>>>) -> String {
    let mut state = state.write().expect("tapoksatan prived medved");
    let response = format!("Hello, {} year old named {}!", age, name);
    state.insert(name, age);
    response

}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![hello])
        .manage(RwLock::new(HashMap::<String,u8, _>::new()))
        .launch();
}
