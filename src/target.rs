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

/*
/// Creates a timestamp in format `+MMM:SS.ssss`.
///
/// This can be used in `Target::write` to create log lines.
#[cfg(features = "std")]
pub fn create_timestamp(duration_since_start: Duration) -> String {
	let minutes = duration_since_start.as_secs() / 60;
	let seconds = duration_since_start.as_secs() % 60;
	let millis = duration_since_start.subsec_millis();

	format!("+{:0>3}:{:0>2}.{:0>4}", minutes, seconds, millis)
}

/// Creates a log line in format `[create_timestamp()][record.level()] (record.target()) record.args()`.
///
/// This can be used in `Target::write` to create log lines.
#[cfg(features = "std")]
pub fn create_log_line(record: &Record, duration_since_start: Duration) -> String {
	format!(
		"[{}][{}] ({}) {}",
		create_timestamp(duration_since_start),
		record.level(),
		record.target(),
		record.args()
	)
}

/// Defines which records to reject for given logging target.
pub struct IgnoreList < C: AsRef<[] > > {
patterns: Vec < String >
}
impl IgnoreList {
	pub fn new(patterns: Vec<String>) -> Self { IgnoreList { patterns } }

	pub fn ignore(&self, record: &Record) -> bool {
		self.patterns.iter().any(|p| record.target().contains(p))
	}
}
impl From<Vec<String>> for IgnoreList {
	fn from(v: Vec<String>) -> Self { IgnoreList::new(v) }
}
impl Default for IgnoreList {
	fn default() -> Self { IgnoreList { patterns: Vec::new() } }
}
*/