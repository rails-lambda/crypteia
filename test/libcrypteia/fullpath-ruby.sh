#!/bin/sh
set -e

export X_CRYPTEIA_SSM=x-crypteia-ssm-path:/crypteia/v5/myapp/envs

ruby -e "puts ENV['X_CRYPTEIA_SSM']"
