#!/bin/sh
set -e

php -r "print(getenv('EMPTY') ? 'FOUND' : 'undefined');"
