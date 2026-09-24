FILENAME=../test/texts/bewiki-20251101-pages-articles-multistream_30M.xml

cargo run --release -- -nc <$FILENAME >output.rs.txt
node ../dist/bin/index.js -nc <$FILENAME >output.txt
