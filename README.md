[![Actions Status](https://github.com/customink/crypteia/actions/workflows/test.yml/badge.svg)](https://github.com/customink/crypteia/actions/workflows/test.yml)

# 🛡 Crypteia

![Node](https://shields.io/badge/x-Node.js-x?logo=node.js&style=plastic&color=339933&label=&labelColor=white) ![Ruby](https://shields.io/badge/x-Ruby-x?logo=ruby&style=plastic&color=CC342D&label=&labelColor=white&logoColor=CC342D) ![PHP](https://shields.io/badge/x-PHP-x?logo=php&style=plastic&color=777BB4&label=&labelColor=white) ![Python](https://shields.io/badge/x-Python-x?logo=python&style=plastic&color=3776AB&label=&labelColor=white)

## Rust Lambda Extension for any Runtime to preload SSM Parameters as Secure Environment Variables!

Super fast and only performed once during your function's initialization, Crypteia turns your serverless YAML from this:

```yaml
Environment:
  Variables:
    SECRET: x-crypteia-ssm:/myapp/SECRET
```

Into real runtime (no matter the lang) environment variables backed by SSM Parameter Store. For example, assuming the SSM Parameter path above returns `1A2B3C4D5E6F` as the value. Your code would return:

```javascript
// node
process.env.SECRET; // 1A2B3C4D5E6F
```

```ruby
# ruby
ENV['SECRET'] # 1A2B3C4D5E6F
```

We do this using our shared object library via the `LD_PRELOAD` environment variable in coordination with our [Lambda Extension](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html) binary file. Unlike other [AWS SSM Lambda Extension Solutions](https://aws.amazon.com/about-aws/whats-new/2022/10/aws-parameters-secrets-lambda-extension/) your code never needs to know where these environment variables come from. See installation & usage sections for more details.

💕 Many thanks to the following projects & people for their work, code, and personal help that made Crypteia possible:

- **[Hunter Madison](https://github.com/hmadison)**: Who taught me about how to use redhook based on Michele Mancioppi's [opentelemetry-injector](https://github.com/mmanciop/opentelemetry-injector) project.
- **[Jake Scott](https://github.com/jakejscott)**: And his [rust-parameters-lambda-extension](https://github.com/jakejscott/rust-parameters-lambda-extension) project which served as the starting point for this project.

## Installation

When building your own Lambda Containers, use both the `crypteia` binary and `libcrypteia.so` shared object files that match your platform. Target platform naming conventions include the following:

- Amazon Linux 2: Uses the `-amzn` suffix.
- Debian, Ubuntu, Etc: Uses the `-debian` suffix.

⚠️ For now our project supports the `x86_64` architecture, but we plan to release `arm64` variants soon. Follow or contribute in our [GitHub Issue](https://github.com/customink/crypteia/issues/5) which tracks this topic.

#### Lambda Containers

You have two options here. The easiest is to use Docker's multi stage builds with our [Extension Containers]([https://github.com/orgs/customink/packages?ecosystem=container&tab=packages&ecosystem=container&q=extension](https://github.com/orgs/customink/packages?repo_name=crypteia&q=extension)) to copy the `/opt` directory matching your platform and Crypteia version number. example below. Remember to use `-debian` vs `-amzn` if you are using your own Linux containers. Or change the version number depending on your needs.

```dockerfile
FROM ghcr.io/customink/crypteia-extension-amzn:latest AS crypteia
FROM public.ecr.aws/lambda/nodejs:16
COPY --from=crypteia /opt /opt
ENV LD_PRELOAD=/opt/lib/libcrypteia.so
```

Alternatively, you can download your platform's binary and shared object file from our [Releases](https://github.com/customink/crypteia/releases) page and place them into your projects Docker build directory. Remember, to remove the platform file suffix. Example:

```dockerfile
RUN mkdir -p /opt/lib
RUN mkdir -p /opt/extensions
COPY crypteia /opt/extensions/crypteia
COPY libcrypteia.so /opt/lib/libcrypteia.so
ENV LD_PRELOAD=/opt/lib/libcrypteia.so
```

If you are using Python you will need to add our Crypteia python package to the `PYTHONPATH` in order for things to "just work". The result of this will be that `os.environ["SECRET"]`, `os.environ.get("SECRET")`, and `os.getenv("SECRET")` will be routed to the `getenv` system call and therefore take advantage of the Crypteia rust extension.

```dockerfile
ENV PYTHONPATH=/opt/crypteia/python
```

⚠️ When building your own Lambda Containers, please make sure [glibc](https://www.gnu.org/software/libc/) is installed since this is used by [redhook](https://github.com/geofft/redhook).

#### Lambda Extension

Our Amazon Linux 2 files can be used within a [Lambda Extension](https://docs.aws.amazon.com/lambda/latest/dg/using-extensions.html) that you can deploy to your own AWS account as a [Lambda Layer](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html). You can use this project to build, publish, and deploy that layer since it has the SAM CLI installed. All you need to do is supply your own S3 bucket. For example:

```shell
aws configure
./amzn/setup
S3_BUCKET_NAME=my-bucket ./package/deploy
```

#### Other Containers

If you are using Crypteia on your own Docker containers without the Lambda Extension mechanics, you can simply set the `ENTRYPOINT` to the Crypteia binary which fetches your environment variables so the shared object preload can use them.

```dockerfile
FROM ghcr.io/customink/crypteia-extension-amzn:latest AS crypteia
FROM ubuntu
COPY --from=crypteia /opt /opt
ENV LD_PRELOAD=/opt/lib/libcrypteia.so
ENTRYPOINT /opt/extensions/crypteia
```

## Usage

First, you will need your secret environment variables setup in [AWS Systems Manager Parameter Store](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html). These can be whatever [hierarchy](https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-hierarchies.html) you choose. Parameters can be any string type. However, we recommend using `SecureString` to ensure your secrets are encrypted within AWS. For example, let's assume the following paramter paths and values exists.

- `/myapp/SECRET` -> `1A2B3C4D5E6F`
- `/myapp/access-key` -> `G7H8I9J0K1L2`
- `/myapp/envs/DB_URL` -> `mysql2://u:p@host:3306`
- `/myapp/envs/NR_KEY` -> `z6y5x4w3v2u1`

Crypteia supports two methods to fetch SSM parameters:

1. `x-crypteia-ssm:` - Single path for a single environment variable.
2. `x-crypteia-ssm-path:` - Path prefix to fetch many environment variables.

Using whatever serverless framework you prefer, setup your function's environment variables using either of the two SSM interfaces from above. For example, here is a environment variables section for an [AWS SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started.html) template that demonstrates all of Crypteia's features.

```yaml
Environment:
  Variables:
    SECRET: x-crypteia-ssm:/myapp/SECRET
    ACCESS_KEY: x-crypteia-ssm:/myapp/access-key
    X_CRYPTEIA_SSM: x-crypteia-ssm-path:/myapp/envs
    DB_URL: x-crypteia
    NR_KEY: x-crypteia
```

When your function initializes, each of the four environmet variables (`SECRET`, `ACCESS_KEY`, `DB_URL`, and `NR_KEY`) will return values from their respective SSM paths.

```javascript
// node
process.env.SECRET;       // 1A2B3C4D5E6F
process.env.ACCESS_KEY;   // G7H8I9J0K1L2
process.env.DB_URL;       // mysql2://u:p@host:3306
process.env.NR_KEY;       // z6y5x4w3v2u1
```

```ruby
# ruby
env["SECRET"];       ## 1A2B3C4D5E6F
env["ACCESS_KEY"];   ## G7H8I9J0K1L2
env["DB_URL"];       ## mysql2://u:p@host:3306
env["NR_KEY"];       ## z6y5x4w3v2u1
```


Here are a few details about the internal implementation on how Crypteia works:

1. When accessing a single parameter path via `x-crypteia-ssm:` the environment variable name available to your runtime is used as is. No part of the parameter path effects the resulting name.
2. When using `x-crypteia-ssm-path:` the environment variable name can be anything and the value is left unchanged.
3. The parameter path hierarchy passed with `x-crypteia-ssm-path:` must be one level deep and end with valid environment variable names. These names must match environement placeholders using `x-crypteia` values.

For security, the usage of `DB_URL: x-crypteia` placeholders ensures that your application's configuration is in full control on which dynamic values can be used with `x-crypteia-ssm-path:`.

Shown below is a simple Node.js 16 function which has the appropriate [IAM Permissions](#iam-permissions) and Crypteia extension via a Lambda Layer installed. Also configured are the needed `LD_PRELOAD` and `SECRET` environment variables. The code of this function log the value of the `process.env.SECRET` which does correctly resolve to the value within SSM Parameter Store.

![Screenshot of the Environment variables in the AWS Lambda Console showing `LD_PRELOAD` to `/opt/lib/libcrypteia.so` and `SECRET` to `x-crypteia-ssm:/myapp/SECRET`.](/images/readme-env-variables.png)

![Screenshot of Code source in the AWS Lambda Console showing the `body` results of `1A2B3C4D5E6F` which is resolved from SSM Parameter Store.](/images/readme-code-results.png)

#### IAM Permissions

Please use AWS' [Restricting access to Systems Manager parameters using IAM policies](https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-access.html) guide for details on what policies your function's IAM Role will need. For an appliction to pull both single parameters as well as bulk paths, I have found the following policy helpful. It assumed the `/myapp` prefix and using AWS default KMS encryption key.

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "ssm:GetParameter",
        "ssm:GetParametersByPath",
        "ssm:GetParameters",
        "ssm:GetParameterHistory",
        "ssm:DescribeParameters"
      ],
      "Resource": "arn:aws:ssm:us-east-1:123456789012:parameter/myapp*",
      "Effect": "Allow"
    },
    {
      "Action": "kms:Decrypt",
      "Resource": "arn:aws:kms:us-east-1:123456789012:key/4914ec06-e888-4ea5-a371-5b88eEXAMPLE",
      "Effect": "Allow"
    }
  ]
}
```

#### Troubleshooting

Crypteia has very verbose logging which enabled by creating an environment variable:

```ruby
CRYPTEIA_DEBUG: true
```

Example of logs:

```log

{"All":"all","ErrorMessage":"","_aws":{"CloudWatchMetrics":[{"Dimensions":[["All","lib"]],"Metrics":[{"Name":"initialized","Unit":"Count"}],"Namespace":"Crypteia"}],"Timestamp":1670424178585},"initialized":1,"lib":"lib"}
{"All":"all","ErrorMessage":"","_aws":{"CloudWatchMetrics":[{"Dimensions":[["All","main"]],"Metrics":[{"Name":"initialized","Unit":"Count"}],"Namespace":"Crypteia"}],"Timestamp":1670424178590},"initialized":1,"main":"main"}
{"All":"all","ErrorMessage":"","_aws":{"CloudWatchMetrics":[{"Dimensions":[["All","main"]],"Metrics":[{"Name":"fetched","Unit":"Count"}],"Namespace":"Crypteia"}],"Timestamp":1670424178831},"fetched":1,"main":"main"}
{"All":"all","ErrorMessage":"","_aws":{"CloudWatchMetrics":[{"Dimensions":[["All","lib"]],"Metrics":[{"Name":"initialized","Unit":"Count"}],"Namespace":"Crypteia"}],"Timestamp":1670424178892},"initialized":1,"lib":"lib"}
{"All":"all","ErrorMessage":"","_aws":{"CloudWatchMetrics":[{"Dimensions":[["All","lib"]],"Metrics":[{"Name":"is_env_file","Unit":"Count"}],"Namespace":"Crypteia"}],"Timestamp":1670424179575},"is_env_file":1,"lib":"lib"}
{"All":"all","ErrorMessage":"","_aws":{"CloudWatchMetrics":[{"Dimensions":[["All","lib"]],"Metrics":[{"Name":"read_env_file","Unit":"Count"}],"Namespace":"Crypteia"}],"Timestamp":1670424179575},"lib":"lib","read_env_file":1}
{"All":"all","ErrorMessage":"","_aws":{"CloudWatchMetrics":[{"Dimensions":[["All","lib"]],"Metrics":[{"Name":"delete_file","Unit":"Count"}],"Namespace":"Crypteia"}],"Timestamp":1670424179575},"delete_file":1,"lib":"lib"}

```

## Development

This project is built for [GitHub Codespcaes](https://github.com/features/codespaces) using the [Development Container](https://containers.dev) specification. Even though Codespaces may not be available to everyone, this project's containers are easy to work for anyone with any editor.

Our development container is based on the [vscode-remote-try-rust](https://github.com/microsoft/vscode-remote-try-rust) demo project. For details on the VS Code Rust development container, have a look at the [container's history](https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust/history). Once you have the repo cloned and setup with a dev container using Codespaces, [VS Code](#using-vs-code), or the [Dev Container CLI](#dev-container-cli), run the following commands. This will install packages, build your project, and run tests without needing to connect to SSM.


```shell
./bin/setup
./bin/test-local
```

If you want to test SSm with your AWS account, the AWS CLI is installed on the dev container. Set it up with your **test credentials** using:

```shell
aws configure
```

You can to develop the Amazon Linux 2 support, This will use Docker in Docker to download AWS SAM & Lambda images to build cryptia using what is present (like glibc) in that environment.

```shell
./amzn/setup
./amzn/test
```

#### Using VS Code

If you have the [Visual Studio Code Dev Container](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension installed you can easily clone this repo locally, use the "Open Folder in Container..." command, and use the integrated terminal for your setup and test commands. Example shown below:

![VS Code showing the "Dev Containers: Open Folder in Container..." command.](/images/readme-devcontainer-open-folder.png)

![VS Code window with the Crypteia project open in a dev container. Shown too are the tests running.](/images/readme-devcontainer-vscode.png)

#### Dev Container CLI

You can use the open-source [Dev Container CLI](https://github.com/devcontainers/cli) to mimic what Codespaces and/or VS Code are doing for you. In this way, you can use different editors. You must have Docker installed. Here are the commands to build the dev container and setup/test the project.

```shell
devcontainer build --workspace-folder .
devcontainer up --workspace-folder .
devcontainer run-user-commands --workspace-folder .
```

```shell
devcontainer exec --workspace-folder . ./bin/setup
devcontainer exec --workspace-folder . ./bin/test-local
```

![Showing Sublime Text on a Mac using the Dev Container CLI to run Crypteia tests.](/images/readme-devcontainer-cli-sublime.png)
