use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

/// Returns the path to the cargo manifest directory.
fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
}

/// Returns the path to the output directory set by cargo.
fn out_dir() -> PathBuf {
    PathBuf::from(env::var_os("OUT_DIR").unwrap())
}

fn main() {
    let manifest = manifest_dir();
    let out = out_dir();

    // parse entity data
    let raids = parse_data::<_, Entity>(manifest.join("data/entities/raids.yml"));
    let fractals = parse_data::<_, Entity>(manifest.join("data/entities/fractals.yml"));

    // bosses
    let boss = raids
        .iter()
        .chain(fractals.iter())
        .map(|(name, entity)| (name.clone(), entity.clone()))
        .collect();

    // generate entity enums
    write_formatted(
        out.join("boss.rs"),
        generate_entity("Boss entities from Raids & Fractals.", "Boss", &boss),
    );
    write_formatted(
        out.join("raidboss.rs"),
        generate_entity("Raid boss entities.", "RaidBoss", &raids),
    );
    write_formatted(
        out.join("fractalboss.rs"),
        generate_entity("Fractal boss entities.", "FractalBoss", &fractals),
    );

    // parse buff data
    let boon = parse_data::<_, Buff>(manifest.join("data/buffs/boon.yml"));
    let food = parse_data::<_, Buff>(manifest.join("data/buffs/food.yml"));
    let util = parse_data::<_, Buff>(manifest.join("data/buffs/util.yml"));

    // merge for all buffs
    let all = boon
        .iter()
        .chain(food.iter())
        .chain(util.iter())
        .map(|(name, buff)| (name.clone(), buff.clone()))
        .collect();

    // generate buff enums
    write_formatted(
        out.join("buff.rs"),
        generate_buff("All Buffs.", "Buff", &all),
    );
    write_formatted(
        out.join("boon.rs"),
        generate_buff("Boon Buffs.", "Boon", &boon),
    );
    write_formatted(
        out.join("food.rs"),
        generate_buff("Food Buffs.", "Food", &food),
    );
    write_formatted(
        out.join("util.rs"),
        generate_buff("Utility Buffs.", "Utility", &util),
    );

    // rerun info
    println!("cargo:rerun-if-changed=data/entities/raids.yml");
    println!("cargo:rerun-if-changed=data/entities/fractals.yml");
    println!("cargo:rerun-if-changed=data/buffs/boon.yml");
    println!("cargo:rerun-if-changed=data/buffs/food.yml");
    println!("cargo:rerun-if-changed=data/buffs/util.yml");
}

/// Buff data entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Buff {
    id: usize,
    name: Option<String>,

    #[serde(default)]
    stats: Vec<String>,

    category: Option<String>,
}

/// Entity data entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entity {
    id: usize,
    name: Option<String>,
    encounter: Option<String>,
    location: Option<String>,
}

/// Parses data from a file.
fn parse_data<P, D>(path: P) -> HashMap<String, D>
where
    D: DeserializeOwned,
    P: AsRef<Path>,
{
    serde_yaml::from_reader(File::open(path.as_ref()).unwrap()).unwrap()
}

// Generates a buff enum from buff data.
fn generate_buff(doc: &str, name: &str, buffs: &HashMap<String, Buff>) -> TokenStream {
    // generate enum name
    let enum_name = format_ident!("{}", name);

    // generate enum
    let raw_enum = generate_enum(
        &enum_name,
        buffs
            .iter()
            .map(|(name, buff)| (format_ident!("{}", name), buff.id)),
    );

    // generate name match
    let name_match = generate_match(buffs.iter().map(|(name, buff)| {
        (
            format_ident!("{}", name),
            buff.name.as_ref().unwrap_or(name),
        )
    }));

    // generate stat match
    let stat_match = generate_match(buffs.iter().map(|(name, buff)| {
        let stats = buff.stats.iter();
        (format_ident!("{}", name), quote! { &[#(#stats),*] })
    }));

    // generate category match
    let category_match = generate_match(buffs.iter().map(|(name, buff)| {
        (
            format_ident!("{}", name),
            buff.category
                .as_ref()
                .map(|category| quote! { Some(#category) })
                .unwrap_or(quote! { None }),
        )
    }));

    // generate full code
    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive, Display, IntoStaticStr, EnumIter, Serialize, Deserialize)]
        #[repr(u32)]
        pub #raw_enum

        impl #enum_name {
            /// Returns the display name of the buff.
            pub fn name(&self) -> &'static str {
                #name_match
            }

            /// Returns the stats applied by the buff.
            ///
            /// The returned slice will be empty if no stats are applied or the stats are not known.
            pub fn stats(&self) -> &[&'static str] {
                #stat_match
            }

            /// Returns the category of the buff.
            pub fn category(&self) -> Option<&'static str> {
                #category_match
            }
        }
    }
}

// Generates an entity enum from entity data.
fn generate_entity(doc: &str, name: &str, entities: &HashMap<String, Entity>) -> TokenStream {
    // generate enum name
    let enum_name = format_ident!("{}", name);

    // generate enum
    let raw_enum = generate_enum(
        &enum_name,
        entities
            .iter()
            .map(|(name, entity)| (format_ident!("{}", name), entity.id)),
    );

    // generate name match
    let name_match = generate_match(entities.iter().map(|(name, entity)| {
        (
            format_ident!("{}", name),
            entity.name.as_ref().unwrap_or(name),
        )
    }));

    // generate encounter match
    let encounter_match = generate_match(entities.iter().map(|(name, entity)| {
        (
            format_ident!("{}", name),
            match &entity.encounter {
                Some(encounter) => quote! { Some(#encounter) },
                None => quote! { None },
            },
        )
    }));

    // generate location match
    let location_match = generate_match(entities.iter().map(|(name, entity)| {
        (
            format_ident!("{}", name),
            match &entity.location {
                Some(location) => quote! { Some(#location) },
                None => quote! { None },
            },
        )
    }));

    // generate full code
    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive, Display, IntoStaticStr, EnumIter, Serialize, Deserialize)]
        #[repr(usize)]
        pub #raw_enum

        impl #enum_name {
            /// Returns the display name of the entity.
            pub fn name(&self) -> &'static str {
                #name_match
            }

            /// Returns the encounter of the entity.
            pub fn encounter(&self) -> Option<&'static str> {
                #encounter_match
            }

            /// Returns the location of the entity.
            pub fn location(&self) -> Option<&'static str> {
                #location_match
            }
        }
    }
}

/// Helper function to generate enums.
fn generate_enum<I>(name: &Ident, kinds: I) -> TokenStream
where
    I: Iterator<Item = (Ident, usize)>,
{
    let kinds = kinds.map(|(ident, id)| {
        let id = Literal::usize_unsuffixed(id);
        quote! { #ident = #id, }
    });
    quote! {
        enum #name {
            #(#kinds)*
        }
    }
}

/// Helper function to generate matches on `self`.
fn generate_match<I, T>(iter: I) -> TokenStream
where
    I: Iterator<Item = (Ident, T)>,
    T: ToTokens,
{
    let matches = iter.map(|(ident, result)| {
        quote! {
            Self::#ident => #result,
        }
    });
    quote! {
        match self {
            #(#matches)*
        }
    }
}

/// Saves a token stream to a file and formats the output.
fn write_formatted<P>(path: P, contents: TokenStream)
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    // convert to string
    let input = contents.to_string();

    // spawn rustfmt
    let manifest = manifest_dir();
    let mut rustfmt = Command::new("rustfmt")
        .arg("--config-path")
        .arg(manifest.join("rustfmt.toml"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // write input
    let mut stdin = rustfmt.stdin.take().unwrap();
    write!(stdin, "{}", input).unwrap();
    drop(stdin);

    // wait & grab output
    let output = rustfmt.wait_with_output().unwrap();
    if !output.status.success() {
        panic!("rustfmt failed to format {:?}", path);
    }
    let formatted = String::from_utf8(output.stdout).unwrap();

    // save to file
    fs::write(path, formatted).unwrap();
}
