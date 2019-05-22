extern crate roggen;
extern crate diesel;
extern crate rand;

use self::roggen::*;
use rand::Rng;

fn main() {
    let connection = establish_connection();

    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        create_post(&connection, "moto", &format!("Moto {}", rng.gen::<u32>()));
        create_post(&connection, "roggen", &format!("Roggen {}", rng.gen::<u32>()));
        create_post(&connection, "tech", &format!("Tech {}", rng.gen::<u32>()));
    }
}
