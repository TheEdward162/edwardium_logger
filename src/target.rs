use std::time::Duration;

use log::{Level, Record};

/// Represents a log target.
pub trait Target {
	#[cfg(feature = "std")]
	type Error: std::error::Error;

	#[cfg(not(feature = "std"))]
	type Error;

	/// Returns the max level this target logs.
	fn level(&self) -> Level;

	/// Whether the target wants to ignore given record.
	///
	/// This method is called before `write` to filter output.
	fn ignore(&self, _record: &Record) -> bool {
		false
	}

	/// Writes record to the target output.
	fn write(&self, duration_since_start: Duration, record: &Record) -> Result<(), Self::Error>;

	/// Flushes target output.
	fn flush(&self) -> Result<(), Self::Error>;
}