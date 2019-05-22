use std::env;

use diesel::PgConnection;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();
        start_pool(env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env"))
    };
}

fn start_pool(url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(20)
        .build(manager)
        .unwrap()
}

pub fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    POOL.clone()
}