//! BOINC integration layer.
//!
//! This module exposes protocol helpers, transport abstraction,
//! high-level read/write APIs, and typed BOINC models.

pub mod api;
pub mod bootstrap;
pub mod models;
pub mod protocol;
pub mod rpc_client;
pub mod transport;
