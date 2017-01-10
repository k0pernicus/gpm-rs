extern crate chrono;
extern crate rustc_serialize;
extern crate toml;
extern crate walkdir;

pub mod configuration;
pub mod file;
pub mod scan;

use toml::{Encoder, Table};

/// The type of the content file is a Table type.
pub type ConfigurationContent = Table;

/// The configuration file is basically a TOML file that contain some informations about local git
/// projects.
pub type ConfigurationFile = Encoder;
