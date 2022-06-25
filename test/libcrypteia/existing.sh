#!/bin/sh
set -e

export EXISTING=existingvalue
export LD_PRELOAD="${LD_PRELOAD:=/workspaces/crypteia/target/release/libcrypteia.so}"

if [ $1 = "ruby" ]; then
  ruby -e "puts(ENV['EXISTING'])"
fi

if [ $1 = "node" ]; then
  node --print "process.env.EXISTING"
fi
