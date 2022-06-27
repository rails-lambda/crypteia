#!/bin/sh
set -e

export EXISTING=existingvalue

node --print "process.env.EXISTING"
