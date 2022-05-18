#[path = "src/data/mod.rs"]
mod data;

#[path = "src/util.rs"]
mod util;

use data::DefData;
use std::{env, fs, path::PathBuf};
use util::parse_jsonc;

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    // parse default definitions
    let content = fs::read_to_string(PathBuf::from(manifest).join("src/data/definitions.json"))
        .expect("failed to read definitions");
    let mut defs: DefData = parse_jsonc(&content).expect("failed to parse definitions");

    // sort alphabetically
    defs.food.sort_by(|a, b| a.name.cmp(&b.name));
    defs.utility.sort_by(|a, b| a.name.cmp(&b.name));

    // save data
    uneval::to_out_dir(defs, "definitions.rs").expect("failed to write definitions data");
}
