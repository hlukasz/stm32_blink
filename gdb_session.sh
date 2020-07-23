#!/bin/bash
if (( $# == 1)); then
        arm-none-eabi-gdb "$1" -x openocd.gdb
else
        echo "Usage:"
        echo "$0 <filename of firmware in ELF format>"
        exit 1
fi
