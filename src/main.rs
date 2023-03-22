use std::collections::HashSet;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};
use std::str;
use std::vec::Vec;
use std::{fs, io};

use anyhow::{anyhow, bail, Result};
use clap::Parser;
use serde::Deserialize;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info, warn};
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

#[derive(Parser)]
struct Args {
    // Whether to print debug logs
    #[arg(short, long)]
    verbose: bool,

    #[arg(default_value_t = String::from("./.asdfler.yml"))]
    filepath: String,
}

fn start_logger(is_verbose: bool) {
    let level = if is_verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    tracing_subscriber::fmt().with_max_level(level).init();
}

// if main returns a Result<T, Error>, then it will exit with a non-zero status
// code for the Err type and print the message.
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    start_logger(args.verbose);

    validate_asdf_exists()?;

    // Confirm we have a valid filepath
    let filepath = fs::canonicalize(&args.filepath)?;
    let filepath = Path::new(&filepath);

    if !filepath.exists() {
        let a = &args.filepath;
        let f = filepath.to_str().unwrap();
        bail!("Unable to find {f} based on given path of {a}");
    }

    let contents = fs::read_to_string(Path::new(&args.filepath))?;

    // By declaring the instance of the Config struct mutable, every field in it
    // is also mutable, and any fields of contained structs.
    //
    // Also should more nicely handle the error of invalid YAML but...
    let mut config: Config = serde_yaml::from_str(&contents)?;

    // Magic! Not really. iter_mut() gives us a mutable reference to the plugin
    // that we're iterating over. If the plugin has a default version, be sure
    // it's added to the list of versions to install.
    for plugin in config.plugins.iter_mut() {
        if let Some(default_version) = &plugin.default_version {
            plugin.versions.insert(default_version.clone());
        }
    }

    for plugin in config.plugins {
        let plugin_name = &plugin.name;
        info!("=== New plugin: {plugin_name}");

        match run_plugin_add(plugin_name) {
            // debug! macro only takes str primitive literals, not Strings.
            // So to get around that, we have it expand the msg String.
            Ok(msg) => debug!("{msg}"),
            Err(msg) => {
                warn!("{msg}");
                warn!("Skipping installing versions for `{plugin_name}");
                continue;
            }
        };

        if plugin.versions.is_empty() {
            info!("No versions configured");
            continue;
        } else {
            for version in plugin.versions {
                if let Err(msg) = install_version(plugin_name, &version) {
                    error!("{msg}");
                    continue;
                };
            }
        }

        if let Some(default_version) = plugin.default_version {
            set_default_version(plugin_name, &default_version);
        };
    }

    Ok(())
}

// Small wrapper around the which::which usage basically. Just ensures we have
// the asdf executable.
fn validate_asdf_exists() -> Result<()> {
    match which(ASDF) {
        Ok(_) => {
            debug!("asdf available");
            Ok(())
        }
        Err(_) => {
            bail!("!!! asdf could not be found in path");
        }
    }
}

// Actually adds the plugin to asdf.
fn run_plugin_add(plugin_name: &str) -> anyhow::Result<String> {
    let output = Command::new(ASDF)
        .args(["plugin", "add", plugin_name])
        .output()?;

    let status = output.status;

    if status.code().is_none() {
        bail!("No status code available");
    }
    let code = status.code().unwrap();

    match code {
        0 => Ok(format!("`{plugin_name}` successfully installed")),
        2 => Ok(format!("`{plugin_name}` is already installed")),
        1 => Err(anyhow!("`{plugin_name}` could not be installed")),
        _ => Err(anyhow!("Something went wrong adding `{plugin_name}")),
    }
}

// Installs a version of a plugin.
fn install_version(plugin_name: &str, version: &String) -> anyhow::Result<()> {
    info!("Starting installation of {version} for {plugin_name}");

    match run_command(Command::new(ASDF).args(["install", plugin_name, version])) {
        Ok(_) => {
            info!("Looks like we installed version {version} of {plugin_name}!");
            Ok(())
        }
        Err(_) => bail!("Unable to install {version} of {plugin_name}"),
    }
}

fn set_default_version(plugin_name: &str, version: &String) {
    let output = run_command(Command::new(ASDF).args(["global", plugin_name, version]));

    if output.is_err() {
        error!("Unable to set default version");
        return;
    }

    match output.unwrap().status.success() {
        true => info!("Successfully set {version} to default version for {plugin_name}"),
        false => warn!("Failed to set {version} to default version for {plugin_name}"),
    };
}

fn run_command(cmd: &mut Command) -> anyhow::Result<Output> {
    let output = cmd.output()?;
    if !output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
    }
    Ok(output)
}
