mod ssm;
use lambda_extension::{service_fn, Error, LambdaEvent, NextEvent};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("[crypteia] Init");
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    // TODO: Pass this data structure to the shared object library somehow.
    let _parameters = ssm::get_envs(env_vars).await.unwrap();
    println!("[crypteia] Fetched environment variables");
    let func = service_fn(parameters_extension);
    lambda_extension::run(func).await
}

async fn parameters_extension(event: LambdaEvent) -> Result<(), Error> {
    match event.next {
        NextEvent::Shutdown(_e) => {
            println!("[crypteia] Shutdown");
        }
        NextEvent::Invoke(_e) => {}
    }
    Ok(())
}
