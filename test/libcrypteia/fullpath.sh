#!/bin/sh
set -e

export X_CRYPTEIA_SSM=x-crypteia-ssm-path:/crypteia/v5/myapp/envs

node --print "process.env.X_CRYPTEIA_SSM"
