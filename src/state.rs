use crate::{config::{Hashing, JwtConfig, ConnectionPool}, utils::DependenciesInject};

#[derive(Clone)]
pub struct AppState {
    pub di_container: DependenciesInject,
    pub jwt_config: JwtConfig,
}

impl AppState {
    pub fn new(pool: ConnectionPool, jwt_secret: &str) -> Self {
        let jwt_config = JwtConfig::new(jwt_secret);
        let hashing = Hashing::new();

        let di_container = DependenciesInject::new(pool, hashing, jwt_config.clone());
        
        Self { di_container, jwt_config }
    }

}