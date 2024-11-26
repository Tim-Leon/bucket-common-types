#![feature(slice_pattern)]
#![feature(slice_split_once)]
#![feature(associated_type_defaults)]
#![feature(allocator_api)]
extern crate core;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;


#[cfg(feature = "unix_timestamp")]
pub mod unix_timestamp;
pub mod util;
pub mod webhook;
pub mod region;
#[cfg(feature = "middleware")]
pub mod middleware;
#[cfg(feature = "key")]
pub mod key;
pub mod storage_engine;
pub mod bucket;
pub mod share;
pub mod user_settings;
pub mod token;
pub mod account;
pub mod metric;
pub mod search;
pub mod validation;


/// Theses are all the supported encoding for files that are uploaded or downloaded.
#[derive(Clone, Eq, PartialEq, strum::Display, strum::EnumString)]
pub enum Encoding {
    LZ4,
    Zstd,
    Brotli,
    Deflate,
    Gzip,
    Custom(String),
}


#[derive(
Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum AvailabilityStatus {
    //TODO: REMOVE?
    Creating,
    Available,
    Deleting,
    Deleted,
    Updating,
    Archiving,
    Restoring,
    Unavailable,
    Unreachable,
    Corrupted,
}

#[derive(
Debug, Clone, Default, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum DownloadFormat {
    #[default]
    Raw,
    Zip,
    Tar,
}
