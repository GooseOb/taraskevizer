#!/bin/bash
FILENAME=../test/texts/bewiki-20251101-pages-articles-multistream_30M.xml
# "--phonetic --jalways" to be fixed in the future
ARGS='-nc'

OUTPUT_RS=output.rs.txt
cargo run --release -- $ARGS <$FILENAME >output.rs.txt

OUTPUT_JS=output.txt
node ../dist/bin/index.js $ARGS <$FILENAME >$OUTPUT_JS
sed -i 's/\/> / \/>/g' $OUTPUT_JS

exec git diff --no-index --color-words output.txt output.rs.txt
