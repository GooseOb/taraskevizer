//! Regenerate the `{p, r}` dictionary JSON files from the Rust constants.
//!
//! ```sh
//! cargo run --example gen_dict_json            # write the files
//! cargo run --example gen_dict_json -- --check # only verify them
//! ```
//!
//! By default the files are written next to the crate sources
//! (`src/dict/data/`). Set `DEST_DIR` to write them elsewhere instead
//! (e.g. the release staging directory), which avoids a copy step:
//!
//! ```sh
//! DEST_DIR=json cargo run --example gen_dict_json
//! ```

use std::{path::PathBuf, process::ExitCode};

use taraskevizer_core::dict::{json::to_json_batches, PHONETIC, WORD_LIST};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/dict/data")
}

/// Destination directory for the generated files.
///
/// Defaults to [`data_dir`]; overridden by the `DEST_DIR` environment
/// variable (empty values are ignored). Relative paths resolve against the
/// process working directory.
fn out_dir() -> PathBuf {
    std::env::var_os("DEST_DIR")
        .filter(|v| !v.is_empty())
        .map_or_else(data_dir, PathBuf::from)
}

fn main() -> ExitCode {
    let check = std::env::args().any(|arg| arg == "--check");
    let dir = out_dir();
    let mut ok = true;

    for (name, json) in [
        ("wordlist.json", to_json_batches(WORD_LIST)),
        ("phonetic.json", to_json_batches(PHONETIC)),
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
        } else {
            let mut dir_ok = true;
            if let Some(parent) = path.parent() {
                if let Err(err) = std::fs::create_dir_all(parent) {
                    println!("cannot create {}: {err}", parent.display());
                    ok = false;
                    dir_ok = false;
                }
            }
            if dir_ok {
                match std::fs::write(&path, &json) {
                    Ok(()) => println!("{name}: written to {}", path.display()),
                    Err(err) => {
                        println!("{name}: cannot write {}: {err}", path.display());
                        ok = false;
                    }
                }
            }
        }
    }

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
