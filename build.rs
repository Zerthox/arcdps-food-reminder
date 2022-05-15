#[path = "src/data/mod.rs"]
mod data;

#[path = "src/util.rs"]
mod util;

use data::DefData;
use std::{env, fs, path::PathBuf};
use util::parse_jsonc;

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let content = fs::read_to_string(PathBuf::from(manifest).join("src/data/definitions.json"))
        .expect("failed to read definitions");
    let defs: DefData = parse_jsonc(&content).expect("failed to parse definitions");

    uneval::to_out_dir(defs, "definitions.rs").expect("failed to write definitions data");
}
