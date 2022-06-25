#!/bin/sh
set -e

export X_CRYPTEIA_SSM=x-crypteia-ssm-path:/crypteia/v5/myapp/envs
export LD_PRELOAD="${LD_PRELOAD:=/workspaces/crypteia/target/release/libcrypteia.so}"

ruby -e "puts(ENV['X_CRYPTEIA_SSM'])"
