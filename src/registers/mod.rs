//! Access to various system and model specific registers.

pub mod control;
pub mod model_specific;
pub mod xcontrol;
pub mod eflags;


pub use crate::instructions::read_eip;
