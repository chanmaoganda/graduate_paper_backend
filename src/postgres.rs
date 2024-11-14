use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{Config, NoTls};

struct PgUser {
    pub username: String,
    pub password: String,
    pub host: String,
    pub database: String,
}

impl PgUser {
    pub fn from_env() -> Self {
        let username = std::env::var("POSTGRES_USER").unwrap();
        let password = std::env::var("POSTGRES_PASSWORD").unwrap();
        let host = std::env::var("POSTGRES_HOST").unwrap();
        let database = std::env::var("POSTGRES_DB").unwrap();

        Self {
            username,
            password,
            host,
            database,
        }
    }
}

pub async fn build_pool() -> anyhow::Result<Pool> {
    let mut pg_config = Config::new();
    let user = PgUser::from_env();
    
    pg_config.user(user.username);
    pg_config.password(user.password);
    pg_config.host(user.host);
    pg_config.dbname(user.database);
    
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();
    Ok(pool)
}