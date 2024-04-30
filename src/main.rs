#[macro_use]
extern crate rocket;
use rocket::fs::{relative, NamedFile};
use std::path::{Path, PathBuf};

#[get("/<path..>", rank = 10)]
pub async fn file(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("static")).join(path);
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![file])
}
