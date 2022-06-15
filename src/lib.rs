use anyhow::Result;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{spawn, task::JoinHandle};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub args: String,
    pub items: Vec<ParameterItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParameterItem {
    pub name: String,
    pub value: String,
}

pub async fn ssm_get_parameter(
    ssm: &aws_sdk_ssm::Client,
    name: String,
    args: String,
) -> Result<Parameter> {
    let mut items: Vec<ParameterItem> = Vec::new();

    let response = ssm
        .get_parameter()
        .name(args.replace("ssm_parameter:", ""))
        .with_decryption(true)
        .send()
        .await;

    match response {
        Ok(response) => {
            if let Some(parameter) = response.parameter {
                items.push(ParameterItem {
                    name: parameter.name.expect("name is required"),
                    value: parameter.value.expect("value is required"),
                });
            }
        }
        Err(error) => {
            eprintln!(
                "[parameters] Error calling ssm:GetParameter. Environment variable: {name} Args: {args} Error: {err}",
                err = error
            );
        }
    }

    Ok(Parameter {
        name: name.to_owned(),
        args: args.to_owned(),
        items,
    })
}

pub async fn ssm_get_parameters_by_path(
    ssm: &aws_sdk_ssm::Client,
    name: String,
    args: String,
) -> Result<Parameter> {
    let mut items: Vec<ParameterItem> = Vec::new();

    let mut token: Option<String> = None;

    loop {
        let response = ssm
            .get_parameters_by_path()
            .path(args.replace("ssm_parameters:", ""))
            .recursive(true)
            .with_decryption(true)
            .set_next_token(token.clone())
            .send()
            .await;

        match response {
            Ok(response) => {
                for parameters in response.parameters {
                    for parameter in parameters {
                        items.push(ParameterItem {
                            name: parameter.name.expect("name is required"),
                            value: parameter.value.expect("value is required"),
                        });
                    }
                }

                if response.next_token == None {
                    break;
                }

                token = response.next_token;
            }
            Err(error) => {
                eprintln!(
                    "[parameters] Error calling ssm:GetParametersByPath. Environment variable: {name} Args: {args} Error: {err}",
                    err = error
                );
                break;
            }
        }
    }

    Ok(Parameter {
        name: name.to_owned(),
        args: args.to_owned(),
        items,
    })
}

pub async fn fetch_parameters(
    vars: HashMap<String, String>,
    ssm: &aws_sdk_ssm::Client,
) -> Result<Vec<Parameter>> {
    let mut results: Vec<Parameter> = Vec::new();

    let mut handles: Vec<JoinHandle<Result<Parameter>>> = Vec::new();

    for (name, args) in vars {
        if args.starts_with("ssm_parameter:") {
            let ssm_clone = ssm.clone();
            handles.push(spawn(async move {
                ssm_get_parameter(&ssm_clone, name, args).await
            }));
        } else if args.starts_with("ssm_parameters:") {
            let ssm_clone = ssm.clone();
            handles.push(spawn(async move {
                ssm_get_parameters_by_path(&ssm_clone, name, args).await
            }));
        }
    }

    let tasks = join_all(handles).await;

    for task in tasks {
        match task {
            Ok(result) => match result {
                Ok(parameter) => {
                    results.push(parameter);
                }
                Err(error) => eprintln!(
                    "[parameters] Parameter error {err}",
                    err = error
                ),
            },
            Err(error) => eprintln!("[parameters] JoinError {err}", err = error),
        }
    }

    Ok(results)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use aws_sdk_ssm::model::ParameterType;
    use serde_json::json;

    #[tokio::test]
    async fn should_parse() -> Result<()> {
        let config = aws_config::load_from_env().await;
        let ssm = aws_sdk_ssm::Client::new(&config);

        ssm.put_parameter()
            .name("/my/parameter".to_owned())
            .value("my-parameter".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;

        ssm.put_parameter()
            .name("/my/path/prefix/value/1".to_owned())
            .value("value-1".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;

        ssm.put_parameter()
            .name("/my/path/prefix/value/2".to_owned())
            .value("value-2".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;

        let vars: HashMap<String, String> = HashMap::from([
            (
                "FOO_PARAM".to_string(),
                "ssm_parameter:/my/parameterx".to_string(),
            ),
            (
                "FOO_PARAMS".to_string(),
                "ssm_parameters:/my/path/prefixx".to_string(),
            ),
        ]);

        let results = fetch_parameters(vars, &ssm)
            .await
            .expect("Should fetch parameters");

        let json = json!(&results);
        println!("Parameters {}", json);

        Ok(())
    }
}
