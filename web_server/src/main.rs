#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap::<u8, u8> = HashMap::new();
    Template::render("index", context)
}


fn main() {
    rocket::ignite().mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
