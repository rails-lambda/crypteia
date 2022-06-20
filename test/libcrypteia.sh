#!/bin/bash
set -e
. ./test/assert.sh

echo "== Testing libcrypteia =="

assert "./test/libcrypteia/existing.sh" \
       "WORLD"

# assert "./test/libcrypteia/empty.sh" \
#        "TEST"

echo "== Testing complete! =="

assert_end examples
