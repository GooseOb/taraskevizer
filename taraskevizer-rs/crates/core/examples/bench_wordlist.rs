use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: bench_wordlist <xml_file>");
        std::process::exit(1);
    };

    let text = std::fs::read_to_string(filepath).expect("read file");
    let cfg = taraskevizer_core::config::TaraskConfig::default();

    // Only test a 64K chunk
    let chunk_size = 64_000.min(text.len());
    let mut end = chunk_size;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let chunk = &text[..end];

    eprintln!("Benchmarking {} chars...", chunk.len());
    let start = Instant::now();
    let _result = taraskevizer_core::tarask(chunk, &cfg);
    let elapsed = start.elapsed();
    eprintln!("Done in {:.3}s", elapsed.as_secs_f64());
}
