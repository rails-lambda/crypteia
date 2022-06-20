#!/bin/sh
set -e

export HELLO=YALL
export LD_PRELOAD=/workspaces/crypteia/target/release/libcrypteia.so

ruby -e "puts(ENV['HELLO'])"
