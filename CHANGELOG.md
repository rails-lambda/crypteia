# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [2.0.1] - 2024-6-13

- Revert base img due to version `GLIBC_2.34' not found error.
  
## [2.0.0] - 2024-6-12

- Fail if parameter not found.

## [1.1.2] - 2023-05-20

- Simple major version docker tags. Ex: 1

## [1.1.0] - 2023-01-18

- Add arm64 support.

## [1.0.0] - 2022-10-26

### Added

 - `ltrace` for debugging
 - Patch Python's `os.environ` if `PYTHONPATH` is set accordingly. Needed for Crypteia's binary to work in Python environments.

### Changed

 - Use `scratch` base instead of `alpine` for smaller lambda extension images.

## [0.94.0] - 2022-10-03

Schedule release. No changes.

## [0.90.0] - 2022-06-27

🎉 Initial Release!

### Added

- Use `x-crypteia-ssm:` - Single path for a single environment variable.
- Use `x-crypteia-ssm-path:` - Path prefix to fetch many environment variables.
- Binary & Shared Object files for both Debian/Ubuntu/Etc & Amazon Linux 2
