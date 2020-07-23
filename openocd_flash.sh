#!/bin/bash
if (( $# != 1 )); then
        echo "Usage:"
        echo "$0 <filename of firmware in ELF format>"
        exit 1
fi

openocd -f openocd.cfg -c "program $1 verify reset exit"
