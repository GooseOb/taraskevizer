#!/bin/bash

executable="$(pwd)/src/bin.ts"

cd `dirname -- "$(readlink -f -- "$BASH_SOURCE")"`

i=1
while [ -d $i ]; do
	cd $i

	cat input.txt | bun $executable -nc > output.txt

	diff output.txt expected.txt

	cd ..

	i=$((i+1))
done
