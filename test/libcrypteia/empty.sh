#!/bin/sh
set -e

export LD_PRELOAD="${LD_PRELOAD:=/workspaces/crypteia/target/release/libcrypteia.so}"

if [ $1 = "ruby" ]; then
  ruby -e "puts(ENV['EMPTY'])"
fi

if [ $1 = "node" ]; then
  node --print "process.env.EMPTY"
fi
