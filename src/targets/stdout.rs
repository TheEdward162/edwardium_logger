use std::io;
use std::io::Write;
use std::time::Duration;

use log::{Level, Record};

use crate::target::Target;

pub struct StdoutTarget {
	level: Level
}
impl StdoutTarget {
	pub const fn new(level: Level) -> Self {
		StdoutTarget {
			level
		}
	}
}
impl Default for StdoutTarget {
	fn default() -> Self {
		StdoutTarget { level: log::Level::Trace }
	}
}
impl Target for StdoutTarget {
	type Error = io::Error;

	fn level(&self) -> Level { self.level }

	fn write(&self, duration_since_start: Duration, record: &Record) -> io::Result<()> {
		let log_line = super::util::LogLine::new(
			duration_since_start.into(),
			record
		);
		writeln!(&mut io::stdout(), "{}", log_line)
	}

	fn flush(&self) -> io::Result<()> {
		io::stdout().flush()
	}
}