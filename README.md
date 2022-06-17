**🚧 🚧 🚧 - U N D E R &nbsp; C O N S T R U C T I O N - 🚧 🚧 🚧**

[![Actions Status](https://github.com/customink/crypteia/actions/workflows/test.yml/badge.svg)](https://github.com/customink/crypteia/actions/workflows/test.yml)

# Crypteia

## Lambda Extension for Secure SSM Parameters as Environment Variables

To-Do List:

- Huge thanks to this guy! https://github.com/jakejscott/rust-parameters-lambda-extension/issues/1
- Ensure lambda lifecycle events correct. Witnessed shutdown errors. Maybe we need some invoke hook to make sure SSM finished work?
- Make sure dev containers work locally with VS Code Remote Development. Add docs below.
- Set raw environment variables.
- Maybe rename to something like cold environments?

## Development Environment

Using Codespaces or VS Code Remote Containers...

- https://github.com/microsoft/vscode-remote-try-rust
- https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust/history

# Building

```sh
./build.sh
```

# Usage

```shell
FOO_PARAM=ssm_parameter:/my/parameter
FOO_PARAM=my-parameter
```

```shell
FOO_PARAMS=ssm_parameters:/my/path/envs
```
