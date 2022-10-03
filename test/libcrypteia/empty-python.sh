#!/bin/sh
set -e

python -c "import os; print(os.environ.get('EMPTY','undefined'))"
