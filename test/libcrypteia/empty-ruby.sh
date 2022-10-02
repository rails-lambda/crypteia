#!/bin/sh
set -e

ruby -e "puts('undefined') if ENV['EMPTY'].nil?"
