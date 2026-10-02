//! Regenerate the `{p, r}` dictionary JSON files from the Rust constants.
//!
//! ```sh
//! cargo run --example gen_dict_json            # write the files
//! cargo run --example gen_dict_json -- --check # only verify them
//! ```

use std::{path::PathBuf, process::ExitCode};

use taraskevizer_core::dict::{json::to_json, PHONETIC, WORD_LIST};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/dict/data")
}

fn main() -> ExitCode {
    let check = std::env::args().any(|arg| arg == "--check");
    let dir = data_dir();
    let mut ok = true;

    for (name, json) in [
        ("wordlist.json", to_json(WORD_LIST)),
        ("phonetic.json", to_json(PHONETIC)),
    ] {
        let path = dir.join(name);
        if check {
            match std::fs::read_to_string(&path) {
                Ok(existing) if existing == json => println!("{name}: ok"),
                Ok(_) => {
                    println!("{name}: differs from {}", path.display());
                    ok = false;
                }
                Err(err) => {
                    println!("{name}: cannot read {}: {err}", path.display());
                    ok = false;
                }
            }
        } else if let Err(err) = std::fs::write(&path, &json) {
            println!("{name}: cannot write {}: {err}", path.display());
            ok = false;
        } else {
            println!("{name}: written");
        }
    }

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
