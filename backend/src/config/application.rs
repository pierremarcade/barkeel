use axum::{extract::DefaultBodyLimit, http::HeaderValue, Router};
use axum::http::Method;
use barkeel_lib::app::Config;
use barkeel_lib::session::CSRFManager;
use dotenvy::dotenv;
use fluent_templates::{FluentLoader, static_loader};
use std::error::Error;
use tera::Tera;
use tower::layer::Layer;
use tower_http::cors::{Any, AllowOrigin, CorsLayer};
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};

#[cfg(feature = "postgres")]
use barkeel_lib::database::postgres::{Connector, Database};
#[cfg(feature = "mysql")]
use barkeel_lib::database::mysql::{Connector, Database};
#[cfg(feature = "sqlite")]
use barkeel_lib::database::sqlite::{Connector, Database};

use crate::config::routes;

// Define a static loader for localization templates
static_loader! {
    pub static LOCALES = {
        locales: "src/locales",
        fallback_language: "en",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}
// Define the Loader struct
pub struct Loader;
// Implement methods for the Loader struct
impl Loader {
    pub async fn init() -> Result<(), Box<dyn Error>> {
        // Load environment variables from the .env file
        dotenv().ok();
        // Initialize the environment logger
        env_logger::init();
		match Self::check_env_vars() {
			Ok(()) => {
				Self::init_server_web().await?;
				Ok(())
			},
			Err(e) => Err(e),
		}   
    }

    async fn init_server_web() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize Tera templates
        let tera = match Self::init_template() {
            Ok(tera) => tera,
            Err(e) => {
                eprintln!("Failed to initialize Tera: {}", e);
                std::process::exit(1);
            }
        };
        // Initialize the database connection
        let database = Self::init_database()?;
        // Create a new CSRF manager instance
        let csrf_manager = CSRFManager::new();
        // Create a configuration object with the database, templates, and CSRF manager
        let config = Config { database: database.clone(), template: tera, csrf_manager };
        // Initialize CORS settings
        let cors = Self::init_cors();
        // Initialize the routes with the configuration
        let routes =  routes::web::routes(config.clone())
        .nest("/api", routes::api::routes(config.clone()));

        let app = NormalizePathLayer::trim_trailing_slash().layer(routes.with_state(config.clone())
        .layer(cors).layer(DefaultBodyLimit::disable()));
        
        let host = std::env::var("HOST")?;
        let listener = tokio::net::TcpListener::bind(host).await?;
        axum::serve(listener, <NormalizePath<Router> as axum::ServiceExt<axum::http::Request<axum::body::Body>>>::into_make_service(app)).await?;

        Ok(())
    }

    fn init_template() -> Result<Tera, Box<dyn std::error::Error>> {
        let mut tera = Tera::default();
        // Register the Fluent localization function with Tera
        tera.register_function("fluent", FluentLoader::new(&*LOCALES));
        // Add raw templates to Tera from the specified file paths
        tera.add_raw_templates(vec![
            ("base.html", include_str!("../app/views/layouts/base.html")),
            ("sidebar.html", include_str!("../app/views/layouts/sidebar.html")),
            ("pagination.html", include_str!("../app/views/pagination.html")),
            ("404.html", include_str!("../app/views/errors/404.html")),
            ("error.html", include_str!("../app/views/errors/error.html")),
        ])?;
        Ok(tera)
    }

    fn init_cors() -> CorsLayer {
        let allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter_map(|s| HeaderValue::from_str(&s).ok())
            .collect::<Vec<_>>();
    
        if allowed_origins.is_empty() {
            CorsLayer::new().allow_origin(Any)
        } else {
            CorsLayer::new().allow_origin(AllowOrigin::list(allowed_origins))
        }
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
    }

    pub fn init_database() -> Result<Database, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let pool = Connector::connect(&database_url)?;
        Ok(Database::new(pool))
    }

	fn check_env_vars() -> Result<(), Box<dyn std::error::Error>> {
		let required_vars = vec!["HOST", "DATABASE_URL"];
		for var in required_vars {
			if std::env::var(var).is_err() {
				return Err(format!("{} variable must be defined", var).into());
			}
		}
		Ok(())
	}
}