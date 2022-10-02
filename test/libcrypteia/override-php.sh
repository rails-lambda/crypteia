#!/bin/sh
set -e

export SECRET=x-crypteia-ssm:/crypteia/v5/myapp/SECRET

php -r "print(getenv('SECRET'));"
