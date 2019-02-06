//! The main implementation crate for the core of Notion.

#![cfg_attr(feature = "universal-docs", feature(doc_cfg))]

extern crate archive;
extern crate cmdline_words_parser;
extern crate console;
extern crate detect_indent;
extern crate dirs;
extern crate envoy;
extern crate fs_extra;
extern crate indicatif;
extern crate lazycell;
#[cfg(feature = "mock-network")]
extern crate mockito;
extern crate readext;
extern crate regex;
extern crate reqwest;
extern crate semver;
extern crate serde_json;
extern crate tempfile;
extern crate term_size;
extern crate toml;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate winfolder;

pub mod config;
mod distro;
pub mod env;
mod event;
pub(crate) mod fs;
pub mod inventory;
pub mod manifest;
pub mod monitor;
pub mod path;
pub mod platform;
mod plugin;
pub mod project;
pub mod session;
pub mod shell;
pub mod shim;
pub mod style;
pub mod tool;
pub mod toolchain;
pub mod version;

extern crate failure;
extern crate failure_derive;
#[macro_use]
extern crate notion_fail;
#[macro_use]
extern crate notion_fail_derive;

#[macro_use]
extern crate cfg_if;
