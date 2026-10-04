#!/bin/bash
FILENAME=../test/texts/bewiki-20261001-pages-articles-multistream_30M.xml
# "--phonetic --jalways" to be fixed in the future
ARGS='-nc'

OUTPUT_RS=output.rs.txt
cargo run --release -- $ARGS <$FILENAME >output.rs.txt

OUTPUT_JS=output.txt
tarask $ARGS <$FILENAME >$OUTPUT_JS
sed -i 's/\/> / \/>/g' $OUTPUT_JS
