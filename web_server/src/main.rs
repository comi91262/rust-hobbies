#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use std::error::Error;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap::<u8, u8> = HashMap::new();
    Template::render("index", context)
}

#[get("/<path..>")]
fn image(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(path)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, image])
        .attach(Template::fairing())
        .launch();
}
