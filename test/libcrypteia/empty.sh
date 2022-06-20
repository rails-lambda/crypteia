#!/bin/sh
set -e

export LD_PRELOAD="${LD_PRELOAD:=/workspaces/crypteia/target/release/libcrypteia.so}"

ruby -e "puts(ENV['EMPTY'])"
