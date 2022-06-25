#!/bin/sh
set -e

export SECRET=x-crypteia-ssm:/crypteia/v5/myapp/SECRET
export LD_PRELOAD="${LD_PRELOAD:=/workspaces/crypteia/target/release/libcrypteia.so}"

ruby -e "puts(ENV['SECRET'])"
