use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let pool_manager = diesel::r2d2::ConnectionManager::new(database_url);
    diesel::r2d2::Pool::new(pool_manager)
}

embed_migrations!("./migrations");

pub fn establish_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = init_pool(&database_url).expect("Failed to create pool");
    let connection = pool.get().expect("Failed to create connection for migrations");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).expect("Failed to run migrations");
    pool
}
