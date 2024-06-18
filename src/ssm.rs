use crate::log;
use anyhow::Result;
use futures::future::join_all;
use std::collections::HashMap;
use tokio::{spawn, task::JoinHandle};
use aws_sdk_ssm::config::BehaviorVersion;

pub async fn get_envs(env_vars: HashMap<String, String>) -> Result<HashMap<String, String>> {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let ssm_client: aws_sdk_ssm::Client = aws_sdk_ssm::Client::new(&sdk_config);
    let mut results: HashMap<String, String> = HashMap::new();
    let mut handles: Vec<JoinHandle<Result<HashMap<String, String>>>> = Vec::new();
    for (name, path) in env_vars {
        if path.starts_with("x-crypteia-ssm:") {
            let ssm_clone = ssm_client.clone();
            handles.push(spawn(async move {
                ssm_get_parameter(&ssm_clone, name, path).await
            }));
        } else if path.starts_with("x-crypteia-ssm-path:") {
            let ssm_clone = ssm_client.clone();
            handles.push(spawn(async move {
                ssm_get_parameters_by_path(&ssm_clone, name, path).await
            }));
        }
    }
    let tasks = join_all(handles).await;
    for task in tasks {
        match task {
            Ok(result) => match result {
                Ok(parameter) => {
                    parameter.into_iter().for_each(|(key, value)| {
                        results.insert(key, value);
                    });
                }
                Err(error) => log::cloudwatch_metric("ssm", "error", true, Some(error.to_string())),
            },
            Err(error) => log::cloudwatch_metric("ssm", "error", true, Some(error.to_string())),
        }
    }
    Ok(results)
}

async fn ssm_get_parameter(
    ssm: &aws_sdk_ssm::Client,
    name: String,
    path: String,
) -> Result<HashMap<String, String>> {
    let mut items: HashMap<String, String> = HashMap::new();
    let response = ssm
        .get_parameter()
        .name(path.replace("x-crypteia-ssm:", ""))
        .with_decryption(true)
        .send()
        .await;
    match response {
        Ok(response) => {
            if let Some(parameter) = response.parameter {
                items.insert(name, parameter.value.unwrap());
            }
        }
        Err(error) => {
            log::cloudwatch_metric(
                "ssm",
                "error",
                true,
                Some(format!(
                    "Error calling ssm:GetParameter. Environment variable: {} Path: {} Error: {}",
                    name, path, error
                )),
            );
        }
    }
    Ok(items)
}

async fn ssm_get_parameters_by_path(
    ssm: &aws_sdk_ssm::Client,
    name: String,
    path: String,
) -> Result<HashMap<String, String>> {
    let mut items: HashMap<String, String> = HashMap::new();
    let mut token: Option<String> = None;
    loop {
        let ssm_path = path.replace("x-crypteia-ssm-path:", "");
        let response = ssm
            .get_parameters_by_path()
            .path(ssm_path.clone())
            .recursive(true)
            .with_decryption(true)
            .set_next_token(token.clone())
            .send()
            .await;
        match response {
            Ok(response) => {
                if let Some(parameters) = response.parameters {
                    for parameter in parameters {
                        let path_prefix = parameter.name.unwrap();
                        let ssm_clone = ssm_path.clone();
                        let ssm_path_replace = ssm_clone + "/";
                        let env_name = path_prefix.replace(ssm_path_replace.as_str(), "");
                        items.insert(env_name, parameter.value.unwrap());
                    }
                }
                if response.next_token == None {
                    break;
                }
                token = response.next_token;
            }
            Err(error) => {
                log::cloudwatch_metric(
                    "ssm",
                    "error",
                    true,
                    Some(format!(
                        "Error calling ssm:GetParametersByPath. Environment variable: {} Path: {} Error: {}",
                        name, path, error
                    )),
                );
                break;
            }
        }
    }
    Ok(items)
}


#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use aws_sdk_ssm::types::ParameterType; // Corrected import
    use std::collections::HashMap;

    #[tokio::test]
    async fn should_parse() -> Result<()> {
        let sdk_config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
        let ssm_client = aws_sdk_ssm::Client::new(&sdk_config);
        ssm_client
            .put_parameter()
            .name("/crypteia/v5/myapp/SECRET".to_owned())
            .value("1A2B3C4D5E6F".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;
        ssm_client
            .put_parameter()
            .name("/crypteia/v5/myapp/access-key".to_owned())
            .value("G7H8I9J0K1L2".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;
        ssm_client
            .put_parameter()
            .name("/crypteia/v5/myapp/envs/DB_URL".to_owned())
            .value("mysql2://u:p@host:3306".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;
        ssm_client
            .put_parameter()
            .name("/crypteia/v5/myapp/envs/NR_KEY".to_owned())
            .value("z6y5x4w3v2u1".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;
        let env_vars: HashMap<String, String> = HashMap::from([
            ("EXISTING".to_string(), "existingvalue".to_string()),
            (
                "SECRET".to_string(),
                "x-crypteia-ssm:/crypteia/v5/myapp/SECRET".to_string(),
            ),
            (
                "ACCESS_KEY".to_string(),
                "x-crypteia-ssm:/crypteia/v5/myapp/access-key".to_string(),
            ),
            (
                "X_CRYPTEIA_SSM".to_string(),
                "x-crypteia-ssm-path:/crypteia/v5/myapp/envs".to_string(),
            ),
            ("DB_URL".to_string(), "x-crypteia".to_string()),
            ("NR_KEY".to_string(), "x-crypteia".to_string()),
        ]);
        let expected: HashMap<String, String> = HashMap::from([
            ("SECRET".to_string(), "1A2B3C4D5E6F".to_string()),
            ("ACCESS_KEY".to_string(), "G7H8I9J0K1L2".to_string()),
            ("DB_URL".to_string(), "mysql2://u:p@host:3306".to_string()),
            ("NR_KEY".to_string(), "z6y5x4w3v2u1".to_string()),
        ]);
        let results = get_envs(env_vars).await.expect("Should fetch parameters");
        assert_eq!(results, expected);
        Ok(())
    }
}