#!/bin/sh
set -e

export EXISTING=existingvalue

php -r "print(getenv('EXISTING'));"
