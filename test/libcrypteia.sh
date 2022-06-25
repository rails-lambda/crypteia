#!/bin/bash
set -e
. ./test/assert.sh

echo "== Testing libcrypteia =="

assert "./test/libcrypteia/existing.sh" \
       "existingvalue"

assert "./test/libcrypteia/override.sh" \
       "1A2B3C4D5E6F"

assert "./test/libcrypteia/empty.sh" \
       ""

assert "./test/libcrypteia/fullpath.sh" \
       "x-crypteia-ssm-path:/crypteia/v5/myapp/envs"

echo "== Testing complete! =="

assert_end examples
