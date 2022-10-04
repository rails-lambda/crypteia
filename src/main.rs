mod log;
mod ssm;
use lambda_extension::{service_fn, Error, LambdaEvent, NextEvent};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const ENV_FILE: &str = "/tmp/crypteia.json";

#[tokio::main]
async fn main() -> Result<(), Error> {
    log::cloudwatch_metric("main", "initialized", false, None);
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    let env_map = ssm::get_envs(env_vars).await.unwrap();
    log::cloudwatch_metric("main", "fetched", false, None);
    write_envs_to_tmp_json(env_map);
    let func = service_fn(parameters_extension);
    lambda_extension::run(func).await
}

async fn parameters_extension(event: LambdaEvent) -> Result<(), Error> {
    match event.next {
        NextEvent::Shutdown(_e) => {
            log::cloudwatch_metric("main", "shutdown", false, None);
        }
        NextEvent::Invoke(_e) => {}
    }
    Ok(())
}

fn write_envs_to_tmp_json(env_map: HashMap<String, String>) {
    let envs_json = serde_json::to_string(&env_map).unwrap();
    let mut file = File::create(ENV_FILE).unwrap();
    file.write_all(envs_json.as_bytes()).unwrap();
}
