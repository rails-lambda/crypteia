use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
    Server,
};
use crypteia::{fetch_parameters, Parameter};
use lambda_extension::{extension_fn, Error, LambdaEvent, NextEvent};
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::spawn;
use tokio::time::{sleep, Duration};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

type Db = Arc<Mutex<Vec<Parameter>>>;

async fn parameters_extension(event: LambdaEvent) -> Result<(), Error> {
    match event.next {
        NextEvent::Shutdown(_e) => {
            println!("[crypteia] Shutdown");
        }
        NextEvent::Invoke(_e) => {
            println!("[crypteia] Invoke event");
        }
    }
    Ok(())
}

async fn index_handler(Extension(db): Extension<Db>) -> impl IntoResponse {
    let parameters = db.lock().unwrap().clone();
    (StatusCode::OK, Json(parameters))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("[crypteia] init");

    let vars: HashMap<String, String> = std::env::vars().collect();

    let config = aws_config::load_from_env().await;
    let ssm: aws_sdk_ssm::Client = aws_sdk_ssm::Client::new(&config);

    let parameters = fetch_parameters(vars, &ssm).await.unwrap();

    println!("[crypteia] fetched: {}", json!(&parameters).to_string());

    let db = Db::new(Mutex::new(parameters));
    let db_clone = db.clone();

    spawn(async move {
        loop {
            sleep(Duration::from_secs(60)).await;

            let vars = std::env::vars().collect();

            let config = aws_config::load_from_env().await;
            let ssm: aws_sdk_ssm::Client = aws_sdk_ssm::Client::new(&config);

            if let Ok(new_parameters) = fetch_parameters(vars, &ssm).await {
                let mut current_parameters = db_clone.lock().unwrap();

                if new_parameters != *current_parameters {
                    println!("[crypteia] Parameters changed!");
                    *current_parameters = new_parameters;
                } else {
                    println!("[crypteia] Have not changed")
                }
            } else {
                println!("[crypteia] Failed to refresh parameters")
            }
        }
    });

    // NOTE: Start a web server on a background task
    spawn(async {
        let app = Router::new().route("/", get(index_handler)).layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(db))
                .into_inner(),
        );
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("[crypteia] listening on: {}", addr);
        Server::bind(&addr).serve(app.into_make_service()).await
    });

    let func = extension_fn(parameters_extension);
    lambda_extension::run(func).await
}
