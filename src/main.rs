use std::collections::HashSet;
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

    let config: Config = serde_yaml::from_str(&FAKE_FILE).unwrap();

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

const FAKE_FILE: &str = r#"
plugins:
  - name: ruby
  - name: elixir
    default_version: 1.14.2-otp-25
  - name: rust
    default_version: 1.68.0
    versions:
      - 1.62.1
      - 1.61.0
  - name: golang
    versions:
      - 1.20.2
      - 1.19.7
"#;
