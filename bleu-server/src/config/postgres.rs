use diesel::{r2d2::ConnectionManager, PgConnection};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn load() -> Pool {
	let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL does not exist!");
	let manager = ConnectionManager::<PgConnection>::new(postgres_url);
	r2d2::Pool::builder().build(manager).unwrap()
}
