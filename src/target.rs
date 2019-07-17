use std::{io, time::Duration};

use log::{Level, Record};

pub mod file;
pub mod stdout;

/// Represents a log target.
pub trait Target {
	/// Returns the max level this target logs.
	fn level(&self) -> Level;

	/// Whether the target wants to ignore given record.
	///
	/// A simple implementation is provided through `IgnoreList`.
	fn ignore(&self, record: &Record) -> bool;

	/// Writes record to the target output.
	///
	/// This function writes straight to the output and doesn't check againts ignore list.
	fn write(&self, record: &Record, duration_since_start: Duration) -> io::Result<()>;

	/// Flushes target output.
	fn flush(&self) -> io::Result<()>;
}

/// Creates a timestamp in format `+MMM:SS.ssss`.
///
/// This can be used in `Target::write` to create log lines.
pub fn create_timestamp(duration_since_start: Duration) -> String {
	let minutes = duration_since_start.as_secs() / 60;
	let seconds = duration_since_start.as_secs() % 60;
	let millis = duration_since_start.subsec_millis();

	format!("+{:0>3}:{:0>2}.{:0>4}", minutes, seconds, millis)
}

/// Creates a log line in format `[create_timestamp()][record.level()] (record.target()) record.args()`.
///
/// This can be used in `Target::write` to create log lines.
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
pub struct IgnoreList {
	patterns: Vec<String>
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
