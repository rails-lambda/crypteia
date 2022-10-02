#!/bin/sh
set -e

export X_CRYPTEIA_SSM=x-crypteia-ssm-path:/crypteia/v5/myapp/envs

php -r "print(getenv('X_CRYPTEIA_SSM'));"
