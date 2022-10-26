#!/bin/sh
set -e

export X_CRYPTEIA_SSM=x-crypteia-ssm-path:/crypteia/v5/myapp/envs

python -c "import os; print(os.environ.get('X_CRYPTEIA_SSM',''))"
