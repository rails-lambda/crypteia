#!/bin/sh
set -e

export SECRET=x-crypteia-ssm:/crypteia/v5/myapp/SECRET

python -c "import os; print(os.environ.get('SECRET',''))"
