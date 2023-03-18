use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::vec::Vec;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    plugins: Vec<Plugin>,
}

#[derive(Debug, Deserialize)]
struct Plugin {
    name: String,

    #[serde(default)]
    default_version: Option<String>,

    #[serde(default)]
    versions: HashSet<String>,
}

fn main() {
    println!("Here we go!");

    let contents = fs::read_to_string(Path::new("./.asdfler.yml"))
        .expect("Should have been able to read the file");

    let config: Config = serde_yaml::from_str(&contents).unwrap();

    for plugin in config.plugins {
        let plugin_name = plugin.name;
        println!("\n\nNew plugin: {plugin_name}");

        match plugin.default_version {
            None => println!("No default version set"),
            Some(version) => println!("Setting default version to {version}"),
        }

        if plugin.versions.is_empty() {
            println!("No versions configured");
        } else {
            for version in plugin.versions {
                println!("Setting up {version}");
            }
        }
    }
}
