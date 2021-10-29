use std::env;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresConfig {
    pool: Pool,
}

impl PostgresConfig {
    pub fn load() -> Self {
        let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL does not exist!");
        let manager = ConnectionManager::<PgConnection>::new(postgres_url);
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        Self {
            pool
        }
    }

    pub fn get_pool(&self) -> Pool {
        self.pool.clone()
    }
}