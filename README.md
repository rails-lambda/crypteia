# Rust parameters lambda extension

BIG CALL OUT

https://github.com/jakejscott/rust-parameters-lambda-extension/issues/1

To-Do List:

* Ensure lambda lifecycle events correct. Witnessed shutdown errors. Maybe we need some invoke hook to make sure SSM finished work?

## Development Environment

Using Codespaces or VS Code Remote Containers...

- https://github.com/microsoft/vscode-remote-try-rust
- https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust/history

# Building

```sh
./build.sh
```

# Parameter configuration

```
| Provider            | Function/Method | IAM Permission                | Environment Variable                         | Use Case                                        |
|---------------------|-----------------|-------------------------------|----------------------------------------------|-------------------------------------------------|
| SSM Parameter Store | ssm_parameter   | ssm:GetParameter              | FOO_PARAM="ssm_parameter:/my/parameter"      | Retrieve a single parameter                     |
| SSM Parameter Store | ssm_parameters  | ssm:GetParametersByPath       | FOO_PARAMS="ssm_parameters:/my/path/prefix"  | Retrieve multiple parameters from a path prefix |
| Secrets Manager     | secret          | secretsmanager:GetSecretValue | FOO_SECRET="secret:my-secret"                | Retrieve a single secret                        |
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
