#!/bin/bash
set -e
. ./test/assert.sh

echo "== Testing libcrypteia =="

assert "./test/libcrypteia/existing.sh ruby" \
       "existingvalue"
assert "./test/libcrypteia/existing.sh node" \
       "existingvalue"

assert "./test/libcrypteia/override.sh ruby" \
       "1A2B3C4D5E6F"
assert "./test/libcrypteia/override.sh node" \
       "1A2B3C4D5E6F"

assert "./test/libcrypteia/empty.sh ruby" \
       ""
assert "./test/libcrypteia/empty.sh node" \
       "undefined"
       
assert "./test/libcrypteia/fullpath.sh ruby" \
       "x-crypteia-ssm-path:/crypteia/v5/myapp/envs"
assert "./test/libcrypteia/fullpath.sh node" \
       "x-crypteia-ssm-path:/crypteia/v5/myapp/envs"

echo "== Testing complete! =="

assert_end examples
