#![deny(clippy::unwrap_used, clippy::expect_used)]
//! `boincrs` library crate.
//!
//! This crate provides:
//! - BOINC RPC transport and protocol helpers
//! - Typed domain models (`Project`, `Task`, `Transfer`)
//! - Read/write API facades for BOINC GUI RPC
//! - TUI application state and controller wiring

pub mod app;
pub mod boinc;
pub mod error;
pub mod ui;
