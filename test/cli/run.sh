#!/bin/bash

executable="$(pwd)/src/bin.ts"

cd `dirname -- "$(readlink -f -- "$BASH_SOURCE")"`/1

cat input.txt | bun $executable -nc > output.txt

diff output.txt expected.txt

cd ..
