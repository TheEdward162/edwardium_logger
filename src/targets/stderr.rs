use std::io;
use std::io::Write;
use std::time::Duration;

use log::{Level, Record};

use crate::target::Target;

pub struct StderrTarget {
	level: Level
}
impl StderrTarget {
	pub const fn new(level: Level) -> Self {
		StderrTarget {
			level
		}
	}
}
impl Default for StderrTarget {
	fn default() -> Self {
		StderrTarget { level: log::Level::Trace }
	}
}
impl Target for StderrTarget {
	type Error = io::Error;

	fn level(&self) -> Level { self.level }

	fn write(&self, duration_since_start: Duration, record: &Record) -> io::Result<()> {
		let log_line = super::util::LogLine::new(
			duration_since_start.into(),
			record
		);
		writeln!(&mut io::stderr(), "{}", log_line)
	}

	fn flush(&self) -> io::Result<()> {
		io::stderr().flush()
	}
}