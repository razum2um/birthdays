use ::config::Config;
use actix_cors::Cors;
use actix_rt::time::sleep;
use actix_web::{
    middleware,
    web::{self, get, put, resource},
    App, HttpServer,
};
use actix_web_prometheus::PrometheusMetricsBuilder;
use std::time::Duration;

use dotenv::dotenv;
use serde::Deserialize;
use tokio_postgres::NoTls;

mod app;
mod errors;
mod handlers;
mod validation;
mod view;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}

#[macro_export]
macro_rules! app {
    ($pool: expr, $schema: expr) => {{
        let cors = Cors::default().allow_any_origin();

        let prometheus = PrometheusMetricsBuilder::new("api")
            .endpoint("/metrics")
            .build()
            .unwrap();

        App::new()
            .app_data(
                web::JsonConfig::default().error_handler(|json_error, _request| {
                    // TODO
                    json_error.into()
                }),
            )
            .app_data(web::Data::new($pool.clone()))
            .app_data(web::Data::new($schema))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .wrap(prometheus.clone())
            .service(resource("/health").to(handlers::health))
            .service(
                resource("/hello/{username}")
                    .route(put().to(handlers::save_birthday))
                    .route(get().to(handlers::birthday)),
            )
    }};
}

#[macro_export]
macro_rules! setup_db {
    ($init_sql: expr, $schema: expr) => {{
        dotenv().ok();

        let config_ = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();

        let config: AppConfig = config_.try_deserialize().unwrap();
        let pool = config.pg.create_pool(None, NoTls).unwrap();
        
        let stmt = include_str!($init_sql).replace("$schema", &$schema);
        let mut conn = pool.get().await;

        while let Err(_) = conn {
            log::error!("cannot connect to db");
            sleep(Duration::from_millis(1000)).await;
            conn = pool.get().await;
        }

        // migrations for dev env
        log::info!("connected to db, applying: {}", $init_sql);
        let _ = conn.unwrap().simple_query(&stmt).await;

        pool
    }};
}

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = setup_db!("../sql/schema.sql", "public");

    log::info!("starting HTTP server at http://0.0.0.0:8080");
    HttpServer::new(move || app!(pool, "public"))
        .workers(2)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::handlers::BirthdayResponse;
    use actix_web::{
        http,
        test::{self, TestRequest},
    };
    use serde_json::json;

    #[macro_export]
    macro_rules! test_app {
        ($schema: expr) => {{
            let pool = setup_db!("../sql/schema_test.sql", $schema);
            app!(pool, $schema)
        }};
    }

    #[macro_export]
    macro_rules! test_get {
        ($uri: expr) => {{
            TestRequest::get()
                .uri($uri)
                .insert_header(http::header::ContentType::json())
                .to_request()
        }};
    }

    #[macro_export]
    macro_rules! test_put (
        ($uri: expr, $body: expr) => ({
            TestRequest::put().uri($uri).set_json($body).to_request()
        })
    );

    #[actix_web::test]
    async fn test_intergation() {
        let app = test::init_service(test_app!("schema0")).await;
        let put_resp = test::call_service(
            &app,
            test_put!("/hello/john", &json!({"dateOfBirth": "2000-02-05"})),
        )
        .await;
        assert_eq!(put_resp.status(), 204);

        let get_resp = test::call_service(&app, test_get!("/hello/john?on=2023-02-05")).await;
        assert!(get_resp.status().is_success());

        let resp: BirthdayResponse = test::read_body_json(get_resp).await;
        assert_eq!(resp.message, "Hello, john! Happy birthday!");

        let get_resp2 = test::call_service(&app, test_get!("/hello/john?on=2023-02-03")).await;
        assert!(get_resp2.status().is_success());

        let resp2: BirthdayResponse = test::read_body_json(get_resp2).await;
        assert_eq!(resp2.message, "Hello, john! Your birthday is in 2 day(s)");
    }

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(test_app!("schema1")).await;
        let resp = test::call_service(&app, test_get!("/health")).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_unknown_username() {
        let app = test::init_service(test_app!("schema2")).await;
        let resp = test::call_service(&app, test_get!("/hello/mike")).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_invalid_birthday_in_future() {
        let app = test::init_service(test_app!("schema3")).await;
        let resp = test::call_service(
            &app,
            test_put!("/hello/elon", &json!({"dateOfBirth": "2099-01-01"})),
        )
        .await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_invalid_date() {
        let app = test::init_service(test_app!("schema4")).await;
        let resp = test::call_service(
            &app,
            test_put!("/hello/kate", &json!({"dateOfBirth": "wtf"})),
        )
        .await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_wrong_post_method() {
        let app = test::init_service(test_app!("schema5")).await;
        let req = TestRequest::post()
            .uri("/hello/helen")
            .set_json(&json!({}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_empty_body() {
        let app = test::init_service(test_app!("schema6")).await;
        let resp = test::call_service(&app, test_put!("/hello/steve", &json!({}))).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_invalid_username() {
        let app = test::init_service(test_app!("schema7")).await;
        let resp = test::call_service(&app, test_put!("/hello/steve123", &json!({}))).await;
        assert!(resp.status().is_client_error());
    }
}
