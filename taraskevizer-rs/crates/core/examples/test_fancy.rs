use regex::Regex;
fn main() {
    // Test $1 backreference
    let re = Regex::new(r"се(?:к?)(ц)ы[ія]").unwrap();
    let result = re.replace_all("секцыя", "сэ$1ыя");
    println!("$1 backreference: {:?}", result);
    println!("expected: {:?}", "сэцыя");

    // Test $0 backreference
    let re2 = Regex::new(r"\p{P}|\p{S}|\d+").unwrap();
    let result2 = re2.replace_all("планета!", " $0 ");
    println!("$0 backreference: {:?}", result2);
    println!("expected: {:?}", "планета ! ");

    // Test $1 in pipeline-style patterns
    let re3 = Regex::new(r" (\p{P}|\p{S}|\d+) ").unwrap();
    let result3 = re3.replace_all(" планета ! ", "$1");
    println!("$1 pipeline: {:?}", result3);
    println!("expected: {:?}", " планета! ");

    // Test \p{P} with Cyrillic - should NOT match
    let re5 = Regex::new(r"\p{P}").unwrap();
    let result5 = re5.replace_all("планета", "X");
    println!("Cyrillic + \\p{{P}}: {:?}", result5);
    println!("expected: {:?}", "планета");
}
