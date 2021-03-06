extern crate ansi_term;
extern crate chrono;
extern crate git2;
extern crate rustc_serialize;
extern crate toml;
extern crate walkdir;

pub mod configuration;
pub mod file;
pub mod git;
pub mod scan;

use toml::{Encoder, Table};

///
/// Static variable to get the name of the git main directory
///
pub static GIT_DIR_NAME: &'static str = ".git";

///
/// Static variable to get the name of the global configuration file
///
pub static CONFIGURATION_FILE_NAME: &'static str = ".gyro";

///
/// Static variable to get the name of the global configuration file copy
///
static CONFIGURATION_FILE_NAME_BUP: &'static str = ".gyro.new";

///
/// Static variable to get the entry name of the configuration part
///
pub static BODY_ENTRY_NAME: &'static str = "config";

///
/// Static variable to get the entry name of watched git repositories
///
pub static WATCHED_ENTRY_NAME: &'static str = "watched";

///
/// Static variable to get the entry name of ignored git repositories
///
pub static IGNORED_ENTRY_NAME: &'static str = "ignored";

///
/// Static variable to get the entry name of git repo groups
///
pub static GROUPS_ENTRY_NAME: &'static str = "groups";

/// The type of the content file is a Table type.
pub type ConfigurationContent = Table;

/// The configuration file is basically a TOML file that contain some informations about local git
/// projects.
pub type ConfigurationFile = Encoder;
