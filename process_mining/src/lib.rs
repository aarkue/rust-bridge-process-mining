#![warn(
    clippy::doc_markdown,
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs
)]
#![doc = include_str!("../README.md")]

///
/// Event Logs (traditional [`EventLog`] and Object-Centric [`OCEL`])
///
pub mod event_log {
    /// Activity projection of event logs
    pub mod activity_projection;
    /// Constants
    pub mod constants;
    /// [`EventLog`] struct and sub-structs
    pub mod event_log_struct;
    /// XES Export
    pub mod export_xes;
    /// XES Import
    pub mod import_xes;
    /// Streaming XES Import
    pub mod stream_xes;
    ///
    /// OCEL2.0 (Object-Centric Event Logs)
    ///
    pub mod ocel {
        /// [`OCEL`] struct and sub-structs
        pub mod ocel_struct;
        #[allow(clippy::single_match)]
        /// Parser for the OCEL2 XML format
        pub mod xml_ocel_import;
    }
    pub use event_log_struct::{
        Attribute, AttributeValue, Attributes, Event, EventLog, Trace, XESEditableAttribute,
    };
    #[cfg(test)]
    mod tests;
}

///
/// Petri nets
///
pub mod petri_net {
    /// [`PetriNet`] struct
    pub mod petri_net_struct;
    /// Export [`PetriNet`] to `.pnml`
    pub mod pnml;
}

use std::fs::File;
use std::io::BufReader;

#[doc(inline)]
pub use event_log::ocel;

#[doc(inline)]
pub use alphappp::full::alphappp_discover_petri_net;

#[doc(inline)]
pub use event_log::import_xes::import_xes_file;

#[doc(inline)]
pub use event_log::import_xes::import_xes_slice;

#[doc(inline)]
pub use event_log::stream_xes::stream_xes_from_path;

#[doc(inline)]
pub use event_log::stream_xes::stream_xes_slice;

#[doc(inline)]
pub use event_log::stream_xes::stream_xes_slice_gz;

#[doc(inline)]
pub use event_log::stream_xes::stream_xes_file;

#[doc(inline)]
pub use event_log::stream_xes::stream_xes_file_gz;

#[doc(inline)]
pub use event_log::export_xes::export_xes_trace_stream_to_file;

#[doc(inline)]
pub use event_log::export_xes::export_xes_event_log_to_file_path;

#[doc(inline)]
pub use event_log::stream_xes::StreamingXESParser;

#[doc(inline)]
pub use event_log::import_xes::XESImportOptions;

#[doc(inline)]
pub use event_log::event_log_struct::EventLog;

#[doc(inline)]
pub use event_log::ocel::ocel_struct::OCEL;

#[doc(inline)]
pub use event_log::ocel::xml_ocel_import::import_ocel_xml;

#[doc(inline)]
pub use event_log::ocel::xml_ocel_import::import_ocel_xml_file;

#[doc(inline)]
pub use event_log::ocel::xml_ocel_import::import_ocel_xml_slice;

#[doc(inline)]
pub use petri_net::petri_net_struct::PetriNet;

#[doc(inline)]
pub use event_log::activity_projection::EventLogActivityProjection;

///
/// Module for the Alpha+++ Process Discovery algorithm
///
pub mod alphappp {
    /// Automatic determining algorithm parameters for Alpha+++
    pub mod auto_parameters;
    /// Alpha+++ Place Candidate Building
    pub mod candidate_building;
    /// Alpha+++ Place Candidate Pruning
    pub mod candidate_pruning;
    /// Full Alpha+++ Discovery algorithm
    pub mod full;
    /// Event Log Repair (Adding artificial activities)
    pub mod log_repair;
}

///
/// Serialize a [`PetriNet`] as a JSON [`String`]
///
pub fn petrinet_to_json(net: &PetriNet) -> String {
    serde_json::to_string(net).unwrap()
}
///
/// Deserialize a [`PetriNet`] from a JSON [`String`]
///
pub fn json_to_petrinet(net_json: &str) -> PetriNet {
    serde_json::from_str(net_json).unwrap()
}

///
/// Serialize [`OCEL`] as a JSON [`String`]
///
/// [`serde_json`] can also be used to convert [`OCEL`] to other targets (e.g., `serde_json::to_writer`)
///
pub fn ocel_to_json(ocel: &OCEL) -> String {
    serde_json::to_string(ocel).unwrap()
}

///
/// Import [`OCEL`] from a JSON [`String`]
///
/// [`serde_json`] can also be used to import [`OCEL`] from other targets (e.g., `serde_json::from_reader`)
///
pub fn json_to_ocel(ocel_json: &str) -> OCEL {
    serde_json::from_str(ocel_json).unwrap()
}

///
/// Import [`OCEL`] from a JSON file given by a filepath
///
/// [`serde_json`] can also be used to import [`OCEL`] from other targets (e.g., `serde_json::from_slice`)
///
pub fn import_ocel_json_from_path(path: &str) -> Result<OCEL, std::io::Error> {
    let reader: BufReader<File> = BufReader::new(File::open(path)?);
    Ok(serde_json::from_reader(reader)?)
}

// pub fn export_log<P: AsRef<Path>>(path: P, log: &EventLog) {
//     let file = File::create(path).unwrap();
//     let writer = BufWriter::new(file);
//     serde_json::to_writer(writer, log).unwrap();
// }

// pub fn export_log_to_string(log: &EventLog) -> String {
//     serde_json::to_string(log).unwrap()
// }

// pub fn export_log_to_byte_vec(log: &EventLog) -> Vec<u8> {
//     serde_json::to_vec(log).unwrap()
// }

// pub fn import_log<P: AsRef<Path>>(path: P) -> EventLog {
//     let file = File::open(path).unwrap();
//     let reader = BufReader::new(file);
//     let log: EventLog = serde_json::from_reader(reader).unwrap();
//     return log;
// }

// pub fn import_log_from_byte_array(bytes: &[u8]) -> EventLog {
//     let log: EventLog = serde_json::from_slice(&bytes).unwrap();
//     return log;
// }

// pub fn import_log_from_str(json: String) -> EventLog {
//     serde_json::from_str(&json).unwrap()
// }
