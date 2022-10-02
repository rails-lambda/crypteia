#!/bin/sh
set -e

export SECRET=x-crypteia-ssm:/crypteia/v5/myapp/SECRET

ruby -e "puts ENV['SECRET']"
