//! This module contains provided implementations of logging targets.

pub mod util;

#[cfg(feature = "file_target")]
pub mod file;
#[cfg(feature = "stdout_target")]
pub mod stdout;
#[cfg(feature = "stderr_target")]
pub mod stderr;

#[cfg(feature = "uart_target")]
pub mod uart;