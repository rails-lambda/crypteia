#!/bin/sh
set -e

if [ -e $CRYPTEIA_ENV_FILE ]; then
    echo "PRESENT"
else
    echo "REMOVED"
fi
