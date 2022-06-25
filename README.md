**🚧 🚧 🚧 - U N D E R &nbsp; C O N S T R U C T I O N - 🚧 🚧 🚧**

[![Actions Status](https://github.com/customink/crypteia/actions/workflows/test.yml/badge.svg)](https://github.com/customink/crypteia/actions/workflows/test.yml)

# 🛡 Crypteia

## Rust Lambda Extension for any Runtime to preload SSM Parameters as Secure Environment Variables!

Super fast and only performaned once during your function's initialization, Crypteia turns your serverless YAML from this:

```yaml
Environment:
  Variables:
    SECRET: x-crypteia-ssm:/myapp/SECRET
```

Into real runtime (no matter the lang) environment variables backed by SSM Parameter Store. For example, assuming the SSM Parameter path above returns `1A2B3C4D5E6F` as the value. Your code would return:

```javascript
process.env.SECRET; // 1A2B3C4D5E6F
```

```ruby
ENV['SECRET'] # 1A2B3C4D5E6F
```

We do this using our lib via `LD_PRELOAD` with [redhook](https://github.com/geofft/redhook) in coordination with our [Lambda Extension](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html) binary. See installation & usage sections for more details.

💕 Many thanks to the following projects & people for their work, code, and personal help that made Crypteia possible:

- **[Hunter Madison](https://github.com/hmadison)**: Who taught me about how to use redhook based on Michele Mancioppi's [opentelemetry-injector](https://github.com/mmanciop/opentelemetry-injector) project.
- **[Jake Scott](https://github.com/jakejscott)**: And his [rust-parameters-lambda-extension](https://github.com/jakejscott/rust-parameters-lambda-extension) project which served as the starting point for this project.

## Installation

🚧 🚧 🚧 TODO: Installation instructions for both `crypteia` binary extensions and `libcrypteia.so` file via `LD_PRELOAD`...

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

```
process.env.SECRET;       // 1A2B3C4D5E6F
process.env.ACCESS_KEY;   // G7H8I9J0K1L2
process.env.DB_URL;       // mysql2://u:p@host:3306
process.env.NR_KEY;       // z6y5x4w3v2u1
```

Here are a few details about the internal implementation on how Crypteia works:

1. When accessing a single parameter path via `x-crypteia-ssm:` the environment variable name available to your runtime is used as is. No part of the parameter path effects the resulting name.
2. When using `x-crypteia-ssm-path:` the environment variable name can be anything and the value is left unchanged.
3. The parameter path hierarchy passed with `x-crypteia-ssm-path:` must be one level deep and end with valid environment variable names. These names must match environement placeholders using `x-crypteia` values.

For security, the usage of `DB_URL: x-crypteia` placeholders ensures that your application's configuration is in full control on which dynamic values can be used with `x-crypteia-ssm-path:`.

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

## Development

🚧 🚧 🚧 TODO: Talk more about Codespaces or VS Code Remote Containers...

- https://github.com/microsoft/vscode-remote-try-rust
- https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust/history

#### Running Tests

Require an AWS account to populate test SSM Parameters. The AWS CLI is installed on the devcontainer. Set it up with your test credentials using:

```shell
$ aws configure
```
