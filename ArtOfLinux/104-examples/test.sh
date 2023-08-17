#!/usr/bin/env bash

variable="abc"

case "$variable" in 
    abc) echo "\$variable = abc" ;;
    xyx) echo "\$variable = xyz" ;;
esac