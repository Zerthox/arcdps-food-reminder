#[path = "src/data.rs"]
mod data;

use data::{parse_jsonc, DefData};
use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let content = fs::read_to_string(PathBuf::from(manifest).join("data/definitions.json"))
        .expect("failed to read definitions");
    let defs: DefData = parse_jsonc(&content).expect("failed to parse definitions");

    uneval::to_out_dir(defs, "definitions.rs").expect("failed to write definitions data");
}
