# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [2.1.0] - 2025-12-02

### Added

- Amazon Linux 2023 support (x86_64 and arm64)
- Architecture auto-detection in build/test scripts

### Changed

- Upgrade dev container from Debian 11 (Bullseye) to Debian 12 (Bookworm)
- Upgrade Debian arm64 test image to Node.js 22 (LTS)
- Update GitHub Actions runners to ubuntu-22.04
- Enable language runtime tests on all architectures
- Pin wrapt to `<1.15.0` for Python 2.7 compatibility

### Fixed

- ARM64 release builds via QEMU emulation setup
- Disk space issues in release workflow

## [2.0.0] - 2024-6-12

- Fix release workflow and update base image
- Update cargo packages
- Improve logging

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
