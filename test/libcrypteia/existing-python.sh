#!/bin/sh
set -e

export EXISTING=existingvalue

python -c "import os; print(os.environ.get('EXISTING',''))"
