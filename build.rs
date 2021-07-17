use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io,
    path::Path,
};

fn main() -> io::Result<()> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // windows bindings
    windows::build! {
        Windows::Win32::Media::Multimedia::timeGetTime,
    }

    // parse buff data
    let boon = parse_data(Path::new(&manifest_dir).join("data/buffs/boon.yml")).unwrap();
    let food = parse_data(Path::new(&manifest_dir).join("data/buffs/food.yml")).unwrap();
    let util = parse_data(Path::new(&manifest_dir).join("data/buffs/util.yml")).unwrap();

    // merge for all buffs
    let all = boon
        .iter()
        .chain(food.iter())
        .chain(util.iter())
        .map(|(name, entry)| (name.clone(), entry.clone()))
        .collect();

    // generate enums
    fs::write(
        Path::new(&out_dir).join("buff.rs"),
        generate_enum("All buffs.", "Buff", &all),
    )?;
    fs::write(
        Path::new(&out_dir).join("boon.rs"),
        generate_enum("Boon buffs.", "Boon", &boon),
    )?;
    fs::write(
        Path::new(&out_dir).join("food.rs"),
        generate_enum("Food buffs.", "Food", &food),
    )?;
    fs::write(
        Path::new(&out_dir).join("util.rs"),
        generate_enum("Utility buffs.", "Utility", &util),
    )?;

    // rerun info
    println!("cargo:rerun-if-changed=data/buffs/boon.yml");
    println!("cargo:rerun-if-changed=data/buffs/food.yml");
    println!("cargo:rerun-if-changed=data/buffs/util.yml");

    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct Entry {
    id: u32,

    name: Option<String>,

    #[serde(default)]
    stats: Vec<String>,

    category: Option<String>,
}

fn parse_data<P>(path: P) -> io::Result<HashMap<String, Entry>>
where
    P: AsRef<Path>,
{
    Ok(serde_yaml::from_reader(File::open(path.as_ref())?).unwrap())
}

fn generate_enum(doc: &str, name: &str, entries: &HashMap<String, Entry>) -> String {
    format!(
"/// {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Display, AsRefStr, EnumProperty)]
#[repr(u32)]
pub enum {} {{
    {}
}}
",
        doc, name,
        entries.iter()
            .map(|(name, entry)| generate_entry(name, entry))
            .collect::<Vec<_>>()
            .join(",\n\n    "),
    )
}

fn generate_entry(name: &str, entry: &Entry) -> String {
    let mut attributes = Vec::new();

    if let Some(display_name) = &entry.name {
        attributes.push(format!("name = \"{}\"", display_name));
    }
    if !entry.stats.is_empty() {
        attributes.push(format!("stats = \"{}\"", entry.stats.join(", ")));
    }
    if let Some(category) = &entry.category {
        attributes.push(format!("category = \"{}\"", category))
    }

    if attributes.is_empty() {
        format!("{} = {}", name, entry.id)
    } else {
        format!(
            "#[strum(props({}))]\n    {} = {}",
            attributes.join(", "),
            name,
            entry.id
        )
    }
}
