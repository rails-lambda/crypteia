#!/bin/sh
set -e

export SECRET=x-crypteia-ssm:/crypteia/v5/myapp/SECRET
export LD_PRELOAD="${LD_PRELOAD:=/workspaces/crypteia/target/release/libcrypteia.so}"

if [ $1 = "ruby" ]; then
  ruby -e "puts(ENV['SECRET'])"
fi

if [ $1 = "node" ]; then
  node --print "process.env.SECRET"
fi
