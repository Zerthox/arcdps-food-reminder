use serde::{de::DeserializeOwned, Deserialize};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    path::Path,
};

fn main() {
    let manifest_var = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let out_var = env::var_os("OUT_DIR").unwrap();
    let manifest_path = Path::new(&manifest_var);
    let out_path = Path::new(&out_var);

    // windows bindings
    windows::build! {
        Windows::{System::VirtualKey, Win32::Media::Multimedia::timeGetTime},
    }

    // parse entity data
    let raids = parse_data::<_, Entity>(manifest_path.join("data/entities/raids.yml"));
    let fractals = parse_data::<_, Entity>(manifest_path.join("data/entities/fractals.yml"));

    // bosses
    let boss = raids
        .iter()
        .chain(fractals.iter())
        .map(|(name, entity)| (name.clone(), entity.clone()))
        .collect();

    // generate entity enums
    fs::write(
        out_path.join("boss.rs"),
        generate_entity("Boss entities from Raids & Fractals.", "Boss", &boss),
    )
    .unwrap();
    fs::write(
        out_path.join("raidboss.rs"),
        generate_entity("Raid boss entities.", "RaidBoss", &raids),
    )
    .unwrap();
    fs::write(
        out_path.join("fractalboss.rs"),
        generate_entity("Fractal boss entities.", "FractalBoss", &fractals),
    )
    .unwrap();

    // parse buff data
    let boon = parse_data::<_, Buff>(manifest_path.join("data/buffs/boon.yml"));
    let food = parse_data::<_, Buff>(manifest_path.join("data/buffs/food.yml"));
    let util = parse_data::<_, Buff>(manifest_path.join("data/buffs/util.yml"));

    // merge for all buffs
    let all = boon
        .iter()
        .chain(food.iter())
        .chain(util.iter())
        .map(|(name, buff)| (name.clone(), buff.clone()))
        .collect();

    // generate buff enums
    fs::write(
        out_path.join("buff.rs"),
        generate_buff("All buffs.", "Buff", &all),
    )
    .unwrap();
    fs::write(
        out_path.join("boon.rs"),
        generate_buff("Boon buffs.", "Boon", &boon),
    )
    .unwrap();
    fs::write(
        out_path.join("food.rs"),
        generate_buff("Food buffs.", "Food", &food),
    )
    .unwrap();
    fs::write(
        out_path.join("util.rs"),
        generate_buff("Utility buffs.", "Utility", &util),
    )
    .unwrap();

    // rerun info
    println!("cargo:rerun-if-changed=data/entities/raids.yml");
    println!("cargo:rerun-if-changed=data/entities/fractals.yml");
    println!("cargo:rerun-if-changed=data/buffs/boon.yml");
    println!("cargo:rerun-if-changed=data/buffs/food.yml");
    println!("cargo:rerun-if-changed=data/buffs/util.yml");
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Buff {
    id: usize,
    name: Option<String>,

    #[serde(default)]
    stats: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entity {
    id: usize,
    name: Option<String>,
    encounter: Option<String>,
    location: Option<String>,
}

fn parse_data<P, D>(path: P) -> HashMap<String, D>
where
    D: DeserializeOwned,
    P: AsRef<Path>,
{
    serde_yaml::from_reader(File::open(path.as_ref()).unwrap()).unwrap()
}

// TODO: simplify match with helper function
// TODO: switch to proc macro?

fn generate_buff(doc: &str, name: &str, buffs: &HashMap<String, Buff>) -> String {
    format!(
"/// {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive, Display, IntoStaticStr, EnumIter, Serialize, Deserialize)]
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
}}",
        doc,
        name,
        buffs
            .iter()
            .map(|(name, buff)| format!("{} = {}", name, buff.id))
            .collect::<Vec<_>>()
            .join(",\n    "),
        name,
        buffs
            .iter()
            .map(|(name, buff)| format!(
                "Self::{} => \"{}\",",
                name,
                buff.name.as_deref().unwrap_or(name)
            ))
            .collect::<Vec<_>>()
            .join("\n            "),
        buffs
            .iter()
            .map(|(name, buff)| format!(
                "Self::{} => &[{}],",
                name,
                buff.stats
                    .iter()
                    .map(|stat| format!("\"{}\"", stat))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .collect::<Vec<_>>()
            .join("\n            ")
    )
}

fn generate_entity(doc: &str, name: &str, entities: &HashMap<String, Entity>) -> String {
    format!(
"/// {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive, Display, IntoStaticStr, EnumIter, Serialize, Deserialize)]
#[repr(usize)]
pub enum {} {{
    {}
}}

impl {} {{
    /// Returns the display name of the entity.
    pub fn name(&self) -> &'static str {{
        match self {{
            {}
        }}
    }}

    /// Returns the encounter of the entity.
    pub fn encounter(&self) -> Option<&'static str> {{
        match self {{
            {}
        }}
    }}

    /// Returns the location of the entity.
    pub fn location(&self) -> Option<&'static str> {{
        match self {{
            {}
        }}
    }}
}}",
        doc,
        name,
        entities
            .iter()
            .map(|(name, entity)| format!("{} = {}", name, entity.id))
            .collect::<Vec<_>>()
            .join(",\n    "),
        name,
        entities
            .iter()
            .map(|(name, entity)| {
                format!(
                    "Self::{} => \"{}\",",
                    name,
                    entity.name.as_deref().unwrap_or(name)
                )
            })
            .collect::<Vec<_>>()
            .join("\n            "),
        entities
            .iter()
            .map(|(name, entity)| {
                format!(
                    "Self::{} => {},",
                    name,
                    match &entity.encounter {
                        Some(encounter) => format!("Some(\"{}\")", encounter),
                        None => format!("none"),
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n            "),
        entities
            .iter()
            .map(|(name, entity)| {
                format!(
                    "Self::{} => {},",
                    name,
                    match &entity.location {
                        Some(location) => format!("Some(\"{}\")", location),
                        None => format!("none"),
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n            "),
    )
}
