#!/bin/bash
FILENAME=../test/texts/bewiki-20261001-pages-articles-multistream_30M.xml
# ARGS="--phonetic --jalways"
ARGS='-nc'

OUTPUT_RS=output.rs.txt
cargo run --release -- $ARGS <$FILENAME >$OUTPUT_RS
