use std::env;

use dotenv::dotenv;
use rocket::Rocket;
use rocket::response::Stream;
use std::fs::File;

use crate::data::read::load_image;
use std::path::Path;

lazy_static! {
    static ref IMG_PATH: String = {
        dotenv().ok();
        env::var("IMAGE_FOLDER").unwrap()
    };
}

#[get("/<id>")]
fn image(id: i32) -> Stream<File> {
    let img = load_image(id).unwrap();
    Stream::chunked(
        File::open(
            img_path(img.file_name)
        ).unwrap(),
        64)
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/img", routes![image])
}

fn img_path<S>(filename: S) -> String where S: ToString {
    let folder: String = IMG_PATH.clone();
    Path::new(&folder).join(&filename.to_string()).to_str().unwrap().to_string()
}