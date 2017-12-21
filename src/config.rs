//! Configuration for `repo-backup`.

use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;
use sec::Secret;

use failure::{Error, ResultExt};
use toml;

/// The overall configuration struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// General configuration options.
    pub general: General,
    /// Settings specific to the `Github` provider.
    pub github: Option<GithubConfig>,
    /// Settings for the `Gitlab` provider.
    pub gitlab: Option<GitlabConfig>,
}

/// General settings used by `repo-backup`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct General {
    /// The root directory to place all downloaded repositories.
    pub dest_dir: PathBuf,
}

/// Github-specific settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GithubConfig {
    /// The API key to use. You will need to [create a new personal access
    /// token][new] and give it the `public_repo` permissions before you can
    /// fetch repos from GitHub.
    ///
    /// [new]: https://github.com/settings/tokens/new
    pub api_key: Secret<String>,
    /// Should we download all starred repos? (default: true)
    #[serde(default = "always_true")]
    pub starred: bool,
    /// Should we download all owned repos? (default: true)
    #[serde(default = "always_true")]
    pub owned: bool,
}

/// Github-specific settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GitlabConfig {
    /// The API key to use.
    pub api_key: Secret<String>,
    /// URL of the Gitlab instance to fetch repositories from.
    pub url: String,
    /// Should we download all repos owned by organisations you are a part of?
    /// (default: false)
    #[serde(default = "always_false")]
    pub organisations: bool,
    /// Should we download all owned repos? (default: true)
    #[serde(default = "always_true")]
    pub owned: bool,
}

fn always_true() -> bool {
    true
}

fn always_false() -> bool {
    false
}

impl Config {
    /// Load a `Config` from some file on disk.
    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Config, Error> {
        let file = file.as_ref();
        debug!("Reading config from {}", file.display());

        let mut buffer = String::new();
        File::open(file)
            .with_context(|_| format!("Unable to open {}", file.display()))?
            .read_to_string(&mut buffer)
            .context("Reading config file failed")?;

        Config::from_str(&buffer)
    }

    /// Load the config directly from a source string.
    pub fn from_str(src: &str) -> Result<Config, Error> {
        toml::from_str(src)
            .context("Parsing config file failed")
            .map_err(Error::from)
    }

    /// Generate an example config.
    pub fn example() -> Config {
        Config {
            general: General {
                dest_dir: PathBuf::from("/srv"),
            },
            github: Some(GithubConfig {
                api_key: String::from("your API key").into(),
                owned: true,
                starred: false,
            }),
            gitlab: Some(GitlabConfig {
                api_key: String::from("your API key").into(),
                url: String::from("https://gitlab.com/"),
                organisations: true,
                owned: true,
            }),
        }
    }

    /// Serialize the `Config` as TOML.
    pub fn as_toml(&self) -> String {
        match toml::to_string_pretty(self) {
            Ok(s) => s,
            Err(e) => {
                panic!("Serializing a Config should never fail. {}", e);
            }
        }
    }
}
