use log::{ Level, Record };

use std::io;

pub mod stdout;
pub mod file;

/// Represents a log target.
pub trait Target {
	/// Returns the max level this target logs.
	fn level(&self) -> Level;

	/// Whether the target wants to ignore given record.
	fn ignore(&self, record: &Record) -> bool;
	/// Writes string to the target output. This doesn't check the ignore list.
	fn write(&self, string: &str) -> io::Result<()>;
	/// Flushes target output.
	fn flush(&self) -> io::Result<()>;
}

/// Defines which records to reject for given logging target.
pub struct IgnoreList {
	patterns: Vec<String>
}
impl IgnoreList {
	pub fn new(patterns: Vec<String>) -> Self {
		IgnoreList {
			patterns
		}
	}

	pub fn ignore(&self, record: &Record) -> bool {
		for pattern in self.patterns.iter() {
			if record.target().contains(pattern) {
				return true;
			}
		}

		false
	}
}
impl Default for IgnoreList {
	fn default() -> Self {
		IgnoreList {
			patterns: Vec::new()
		}
	}
}