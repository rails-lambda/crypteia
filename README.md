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

We do this using our shared object library via the `LD_PRELOAD` environment variable in coordination with our [Lambda Extension](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html) binary file. See installation & usage sections for more details.

💕 Many thanks to the following projects & people for their work, code, and personal help that made Crypteia possible:

- **[Hunter Madison](https://github.com/hmadison)**: Who taught me about how to use redhook based on Michele Mancioppi's [opentelemetry-injector](https://github.com/mmanciop/opentelemetry-injector) project.
- **[Jake Scott](https://github.com/jakejscott)**: And his [rust-parameters-lambda-extension](https://github.com/jakejscott/rust-parameters-lambda-extension) project which served as the starting point for this project.

## Installation

When building your own Lambda Containers, use both the `crypteia` binary and `libcrypteia.so` shared object files that match your platform. Target platform naming conventions include the following:

- Amazon Linux 2: Uses the `-amzn` suffix.
- Debian, Ubuntu, Etc: Uses the `-debian` suffix.

⚠️ For now our project supports the `x86_64` architecture, but we plan to release `arm64` variants soon. Follow or contribute in our [GitHub Issue](https://github.com/customink/crypteia/issues/5) which tracks this topic.

#### Lambda Containers

You have two options here. The easiest is to use Docker's multi stage builds with our [Extension Containers](https://github.com/orgs/customink/packages?ecosystem=container&tab=packages&ecosystem=container&q=extension) to copy the `/opt` directory matching your platform and Crypteia version number. example below. Remember to use `-debian` vs `-amzn` if you are using your own Linux containers. Or change the version number depending on your needs.

```dockerfile
FROM ghcr.io/customink/crypteia-extension-amzn:0.90.0 AS crypteia
FROM public.ecr.aws/lambda/nodejs:16
COPY --from=crypteia /opt /opt
```

Alternatively, you can download your platform's binary and shared object file from our [Releases](https://github.com/customink/crypteia/releases) page and place them into your projects Docker build directory. Remember, to remove the platform file suffix. Example:

```dockerfile
RUN mkdir -p /opt/lib
RUN mkdir -p /opt/extensions
COPY crypteia /opt/extensions/crypteia
COPY libcrypteia.so /opt/lib/libcrypteia.so
ENV LD_PRELOAD=/opt/lib/libcrypteia.so
```

⚠️ When building your own Lambda Containers, please make sure [glibc](https://www.gnu.org/software/libc/) is installed since this is used by [redhook](https://github.com/geofft/redhook).

#### Lambda Layer

Our Amazon Linux 2 files can be used within a [Lambda Layer](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html) that you can deploy to your own AWS account. You can use this project to build, publish, and deploy that layer since it has the SAM CLI installed. All you need to do is supply your own S3 bucket. For example:

```shell
aws configure
./amzn/setup
S3_BUCKET_NAME=my-bucket ./layer/deploy
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

#### Lambda Layer

Shown below is a simple Node.js 16 function which has the appropriate [IAM Permissions](#iam-permissions) and Crypteia Lambda Layer added. Also configured are the needed `LD_PRELOAD` and `SECRET` environment variables. The code of this function log the value of the `process.env.SECRET` which does correctly resolve to the value within SSM Parameter Store.

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
      "Resource": "*arn:aws:kms:us-east-1:123456789012:key/4914ec06-e888-4ea5-a371-5b88eEXAMPLE*",
      "Effect": "Allow"
    }
  ]
}
```

## Development

This project is built for [GitHub Codespcaes](https://github.com/features/codespaces) which may not be available to everyone. Thankfully you can use this same devcontainer.json specification automatically with [VS Code Remote Development](https://code.visualstudio.com/docs/remote/remote-overview) which allows you to clone this repo and [open the folder in a container](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container).

Our development container is based on the [vscode-remote-try-rust](https://github.com/microsoft/vscode-remote-try-rust) demo project. For details on the VS Code Rust development containers, have a look here: https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust/history. Once you have the repo cloned or setup in a development container, run the following command. This will install and build your project.

```shell
./bin/setup
```

Optionally, you can setup/build the Amazon Linux 2 files. This will use Docker in Docker to download AWS SAM & Lambda images to build cryptia using what is present (like glibc) in those environments.

```shell
./amzn/setup
```

#### Running Tests

Requires an AWS account to populate test SSM Parameters. The AWS CLI is installed on the devcontainer. Set it up with your **test credentials** using:

```shell
aws configure
```

Once complete, you can run the tests using the following command. If you make changes to the code, make sure to run `bin/setup` again whick will run cargo build for you.

```shell
./bin/test
```

Again, if you are working on the Amazon Linux 2 project files, assuming you have already run `./amzn/setup`, you can run the same tests above using that environment.

```shell
./amzn/test
```
