# Rust parameters lambda extension

# Installing

```sh
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or update rust to the latest version
# rustup update

# Add target so we can cross compile for x86 Lambdas
rustup target add x86_64-unknown-linux-musl

# Install dependencies
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install zip musl-tools -y
```

# Building

```sh
./build.sh
```

# Deploying

```sh
cd cdk
cdk bootstrap
cdk deploy
```

# Parameter configuration

```
| Provider            | Function/Method | IAM Permission                | Environment Variable                         | Use Case                                        |
|---------------------|-----------------|-------------------------------|----------------------------------------------|-------------------------------------------------|
| SSM Parameter Store | ssm_parameter   | ssm:GetParameter              | FOO_PARAM="ssm_parameter:/my/parameter"      | Retrieve a single parameter                     |
| SSM Parameter Store | ssm_parameters  | ssm:GetParametersByPath       | FOO_PARAMS="ssm_parameters:/my/path/prefix"  | Retrieve multiple parameters from a path prefix |
| Secrets Manager     | secret          | secretsmanager:GetSecretValue | FOO_SECRET="secret:my-secret"                | Retrieve a single secret                        |
| DynamoDB            | ddb_item        | dynamodb:GetItem              | FOO_ITEM="ddb_item:table=my-table&pk=A&sk=B" | Retrieve an item from a DynamoDB table          |
| DynamoDB            | ddb_query       | dynamodb:Query                | FOO_QUERY="ddb_query:table=my-table&pk=A"    | Retrieve multiple items from a DynamoDB table   |
```

```json
[
  {
    "name": "FOO_PARAM",
    "args": "ssm_parameter:/my/parameter",
    "items": [
      {
        "name": "/my/parameter",
        "value": "my-parameter"
      }
    ]
  },
  {
    "name": "FOO_PARAMS",
    "args": "ssm_parameters:/my/path/prefix",
    "items": [
      {
        "name": "/my/path/prefix/value/1",
        "value": "value-1"
      },
      {
        "name": "/my/path/prefix/value/2",
        "value": "value-1"
      }
    ]
  },
  {
    "name": "FOO_SECRET",
    "args": "secret:my-secret",
    "items": [
      {
        "name": "my-secret",
        "value": "secret-value"
      }
    ]
  },
  {
    "name": "FOO_ITEM",
    "args": "ddb_item:table=my-table&pk=A&sk=B",
    "items": [
      {
        "name": "table=my-table&pk=A&sk=B",
        "value": "{ \"pk\": \"A\", \"sk\": \"B\" }"
      }
    ]
  }
]
```
