//! Structs and datatypes used to represent facts and data in a Gedcom file

// holy wow, this data format is heteronormative af...

#![allow(missing_docs)]

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

pub mod event;
pub use event::{Event, EventType};

mod address;
pub use address::*;

type Xref = String;

// top-level record types
mod header;
pub use header::*;

mod individual;
pub use individual::*;

mod family;
pub use family::*;

mod submitter;
pub use submitter::*;

mod source;
pub use source::*;

// TODO
/// Multimedia item
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Media {}

/// Data repository, the `REPO` tag
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Repository {
    /// Optional reference to link to this repo
    pub xref: Option<Xref>,
    /// Name of the repository
    pub name: Option<String>,
    /// Physical address of the data repository
    pub address: Option<Address>,
}

/// Citation linking a genealogy fact to a data `Source`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct SourceCitation {
    /// Reference to the `Source`
    pub xref: Xref,
    /// Page number of source
    pub page: Option<String>,
}

/// Citation linking a `Source` to a data `Repository`
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct RepoCitation {
    /// Reference to the `Repository`
    pub xref: Xref,
    /// Call number to find the source at this repository
    pub call_number: Option<String>,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct CustomData {
    pub tag: String,
    // TODO: this can sometimes be empty b/c there are deeper levels.
    pub value: Option<String>,
}
