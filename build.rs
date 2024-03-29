#[path = "src/data/structs.rs"]
mod structs;

#[path = "src/util.rs"]
mod util;

use std::{env, fs, path::PathBuf};
use structs::DefData;
use util::parse_jsonc;

const DEF_PATH: &str = "src/data/definitions.json";

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    // parse default definitions
    println!("cargo:rerun-if-changed={DEF_PATH}");
    let content = fs::read_to_string(PathBuf::from(manifest).join(DEF_PATH))
        .expect("failed to read definitions");
    let mut defs: DefData = parse_jsonc(&content).expect("failed to parse definitions");

    // sort alphabetically
    defs.food.sort_by(|a, b| a.name.cmp(&b.name));
    defs.utility.sort_by(|a, b| a.name.cmp(&b.name));

    // save data
    uneval::to_out_dir(defs, "definitions.rs").expect("failed to write definitions data");
}
