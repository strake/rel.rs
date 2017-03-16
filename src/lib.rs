//! Relations

#![no_std]

pub mod eq;
pub mod ord;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Core;
