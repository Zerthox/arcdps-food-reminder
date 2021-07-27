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
        Windows::{System::VirtualKey, Win32::Media::Multimedia::timeGetTime},
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
#[serde(deny_unknown_fields)]
struct Entry {
    id: u32,

    name: Option<String>,

    #[serde(default)]
    stats: Vec<String>,
}

fn parse_data<P>(path: P) -> io::Result<HashMap<String, Entry>>
where
    P: AsRef<Path>,
{
    Ok(serde_yaml::from_reader(File::open(path.as_ref())?).unwrap())
}

fn generate_enum(doc: &str, name: &str, entries: &HashMap<String, Entry>) -> String {
    // TODO: simplify match with helper function
    format!(
        "/// {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Display, IntoStaticStr, EnumIter)]
#[repr(u32)]
pub enum {} {{
    {}
}}

impl {} {{
    /// Returns the display name of the buff.
    pub fn name(&self) -> &'static str {{
        match self {{
            {}
        }}
    }}

    /// Returns the stats applied by the buff.
    ///
    /// The returned slice will be empty if no stats are applied or the stats are not known.
    pub fn stats(&self) -> &[&'static str] {{
        match self {{
            {}
        }}
    }}
}}
",
        doc,
        name,
        entries
            .iter()
            .map(|(name, entry)| format!("{} = {}", name, entry.id))
            .collect::<Vec<_>>()
            .join(",\n    "),
        name,
        entries
            .iter()
            .map(|(name, entry)| format!(
                "Self::{} => \"{}\",",
                name,
                entry.name.as_deref().unwrap_or(name)
            ))
            .collect::<Vec<_>>()
            .join("\n            "),
        entries
            .iter()
            .map(|(name, entry)| format!(
                "Self::{} => &[{}],",
                name,
                entry
                    .stats
                    .iter()
                    .map(|stat| format!("\"{}\"", stat))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .collect::<Vec<_>>()
            .join("\n            "),
    )
}
