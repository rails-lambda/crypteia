#!/bin/bash
set -e
. ./test/assert.sh

export CRYPTEIA_ENV_FILE="/tmp/crypteia.json"
export BUILD_ARCH="${BUILD_ARCH:=debian}"
export LD_PRELOAD="${LD_PRELOAD:=$PWD/build/libcrypteia-${BUILD_ARCH}.so}"

echo "== Simulating crypteia binary JSON write =="
echo '{
  "SECRET": "1A2B3C4D5E6F",
  "ACCESS_KEY": "G7H8I9J0K1L2", 
  "DB_URL": "mysql2://u:p@host:3306",
  "NR_KEY": "z6y5x4w3v2u1"
}' > $CRYPTEIA_ENV_FILE

echo "== Testing libcrypteia =="

assert "./test/libcrypteia/envfile.sh" \
       "PRESENT"

assert "./test/libcrypteia/existing.sh" \
       "existingvalue"

assert "./test/libcrypteia/envfile.sh" \
       "PRESENT" "Because existing values does not trigger libcrypteia code."

assert "./test/libcrypteia/override.sh" \
       "1A2B3C4D5E6F"

assert "./test/libcrypteia/envfile.sh" \
       "REMOVED" "Because first x-crypteia access."

assert "./test/libcrypteia/empty.sh" \
       "undefined"
       
assert "./test/libcrypteia/fullpath.sh" \
       "x-crypteia-ssm-path:/crypteia/v5/myapp/envs" \
       "Because not replaced by a single env var."

echo "== Testing complete! =="

assert_end examples
