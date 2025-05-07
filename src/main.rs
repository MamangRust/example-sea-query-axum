use dotenv::dotenv;
use example_sea_query::config::{Config, ConnectionManager};
use example_sea_query::handler::AppRouter;
use example_sea_query::state::AppState;
use example_sea_query::utils::tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing();

    let config = Config::init();

    let db_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("Error initializing database connection pool");

    let port = config.port;

    let state = AppState::new(db_pool, &config.jwt_secret);

    println!("🚀 Server started successfully");

    AppRouter::serve(port, state).await
}
