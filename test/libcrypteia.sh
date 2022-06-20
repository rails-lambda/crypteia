#!/bin/bash
set -e
. ./test/assert.sh

echo "== Testing libcrypteia =="

assert "./test/libcrypteia/existing.sh" \
       "WORLD"

echo "== Testing complete! =="

assert_end examples
