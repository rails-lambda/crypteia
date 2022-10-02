#!/bin/sh
set -e

export EXISTING=existingvalue

ruby -e "puts ENV['EXISTING']"
