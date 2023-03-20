use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::vec::Vec;

use serde::Deserialize;
use which::which;

const ASDF: &str = "asdf";

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

    validate_asdf_exists();

    let contents = fs::read_to_string(Path::new("./.asdfler.yml"))
        .expect("Should have been able to read the file");

    let config: Config = serde_yaml::from_str(&contents).unwrap();

    for plugin in config.plugins {
        let plugin_name = &plugin.name;
        println!("\n\nNew plugin: {plugin_name}");

        run_plugin_add(&plugin_name);

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

fn validate_asdf_exists() {
    match which(ASDF) {
        Ok(_) => println!("asdf available"),
        Err(_) => {
            println!("!!! asdf could not be found in path");
            std::process::exit(1);
        }
    }
}

fn run_plugin_add(plugin_name: &str) {
    let output = Command::new(ASDF)
        .args(["plugin", "add", &plugin_name])
        .output()
        .expect("!!! Something went very wrong");

    // Print the output so the user sees it. Likely we just want this
    // for version installs since thsoe can be wordy.
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{stdout}");

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{stderr}");

    let status = output.status;
    let code = status
        .code()
        .expect("!!! Unexpectedly terminated by signal");

    if code == 2 {
        println!("`{plugin_name}` is already installed");
    } // Do a code == 1 here to tell the user it failed
}
