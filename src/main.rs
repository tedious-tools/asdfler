use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str;
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

        let plugin_add_result = run_plugin_add(&plugin_name);

        match plugin_add_result {
            // println! macro only takes str primitive literals, not Strings.
            // So to get around that, we have it expand whatever we pass it, which
            // happens to be a String.
            Ok(msg) => println!("{}", msg),
            Err(msg) => {
                println!("{}", msg);
                println!("Skipping installing versions for `{plugin_name}");
                continue;
            }
        };

        // There is likely a way to do this by making the struct mutable, but
        // I'm not sure what it is and these sets are small enough that cloning
        // is just easier to keep moving forward.
        let mut versions_to_install = plugin.versions.clone();

        // We need this to throw an unwrapped default into the hashset of versions
        // and then to check again later for if we need to actaully set default version.
        let default_for_later = plugin.default_version.clone();

        if let Some(default_version) = plugin.default_version {
            versions_to_install.insert(default_version);
        }

        if versions_to_install.is_empty() {
            println!("=== No versions configured");
            continue;
        } else {
            for version in versions_to_install {
                install_version(&plugin_name, &version);
            }
        }

        if let Some(default_version) = default_for_later {
            set_default_version(&plugin_name, &default_version);
        };
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

// This function returns a result, but! It still panics in some specific cases.
// Namely, something very unexpected running theh plugin add command, like a segfault,
// and if the command is terminated by a signal somehow.
// This is probably a bad practice but for now I do not care.
fn run_plugin_add(plugin_name: &str) -> Result<String, String> {
    let output = Command::new(ASDF)
        .args(["plugin", "add", &plugin_name])
        .output()
        .expect("!!! Something went very wrong");

    let status = output.status;
    let code = status
        .code()
        .expect("!!! Unexpectedly terminated by signal");

    match code {
        0 => Ok(format!("`{plugin_name}` successfully installed")),
        2 => Ok(format!("`{plugin_name}` is already installed")),
        1 => Err(format!("`{plugin_name}` could not be installed")),
        _ => Err(format!("Something went wrong adding `{plugin_name}")),
    }
}

fn install_version(plugin_name: &str, version: &String) {
    println!("\n=== Starting installation of {version} for {plugin_name}");
    // TODO: REMOVE THIS
    if version == "do.not.install" {
        println!("Unable to install version {version} for `{plugin_name}`");
        return;
    }

    let output = Command::new(ASDF)
        .args(["install", &plugin_name, &version])
        .output()
        .expect("!!! Something went very wrong");

    let stdout = output.stdout;
    println!("{}", str::from_utf8(&stdout).unwrap().trim());

    let stderr = output.stderr;
    let stderr = str::from_utf8(&stderr).unwrap().trim();

    if stderr != "" {
        println!("{}", stderr);
    }

    println!("=== Looks like we installed version {version} of {plugin_name}!");
}

fn set_default_version(plugin_name: &str, version: &String) {
    println!("\n=== Setting default version of {plugin_name} to {version}");

    let status = Command::new(ASDF)
        .args(["global", &plugin_name, &version])
        .status()
        .expect("!!! Something went wrong setting the default version");

    match status.success() {
        true => println!("=== Successfully set {version} to default version for {plugin_name}"),
        false => println!("=== Failed to set {version} to default version for {plugin_name}"),
    };
}
