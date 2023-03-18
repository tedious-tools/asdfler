use std::collections::HashSet;
use std::vec::Vec;

#[derive(Debug)]
struct Config {
    plugins: Vec<Plugin>,
}

#[derive(Debug)]
struct Plugin {
    name: String,
    default_version: DefaultVersion,
    versions: HashSet<String>,
}

#[derive(Debug)]
enum DefaultVersion {
    Data(String),
    None,
}

// impl DefaultVersion {
//     fn default() -> Self {
//         DefaultVersion::None
//     }
// }

fn main() {
    let config = Config {
        plugins: vec![
            Plugin {
                name: String::from("ruby"),
                default_version: DefaultVersion::None,
                versions: HashSet::new(),
            },
            Plugin {
                name: String::from("elixir"),
                default_version: DefaultVersion::Data(String::from("1.14.2-otp-25")),
                versions: HashSet::new(),
            },
            Plugin {
                name: String::from("rust"),
                default_version: DefaultVersion::Data(String::from("1.68.0")),
                versions: HashSet::from([String::from("1.62.1"), String::from("1.61.0")]),
            },
        ],
    };

    println!("Here we go!");

    for plugin in config.plugins {
        let plugin_name = plugin.name;
        println!("\n\nNew plugin: {plugin_name}");

        match plugin.default_version {
            DefaultVersion::None => println!("No default version set"),
            DefaultVersion::Data(version) => println!("Setting default version to {version}"),
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
