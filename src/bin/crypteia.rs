use crypteia::fetch_parameters;
use lambda_extension::{extension_fn, Error, LambdaEvent, NextEvent};
use serde_json::json;
use std::collections::HashMap;

async fn parameters_extension(event: LambdaEvent) -> Result<(), Error> {
    match event.next {
        NextEvent::Shutdown(_e) => {
            println!("[crypteia] Shutdown");
        }
        NextEvent::Invoke(_e) => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("[crypteia] init");
    let vars: HashMap<String, String> = std::env::vars().collect();
    let config = aws_config::load_from_env().await;
    let ssm: aws_sdk_ssm::Client = aws_sdk_ssm::Client::new(&config);
    let parameters = fetch_parameters(vars, &ssm).await.unwrap();
    println!("[crypteia] fetched: {}", json!(&parameters));
    let func = extension_fn(parameters_extension);
    lambda_extension::run(func).await
}
