#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 [command_line_arguments...]"
  exit 1
fi

cargo compete test "$@"
