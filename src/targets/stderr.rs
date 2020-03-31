use std::{io, io::Write, time::Duration};

use log::{Level, Record};

use crate::target::Target;

#[cfg(feature = "colored_stderr_output")]
use super::util::colored_logline::ColoredLogLine as LogLine;

#[cfg(not(feature = "colored_stderr_output"))]
use super::util::LogLine;

pub struct StderrTarget {
	level: Level
}
impl StderrTarget {
	pub const fn new(level: Level) -> Self { StderrTarget { level } }
}
impl Default for StderrTarget {
	fn default() -> Self { StderrTarget { level: log::Level::Trace } }
}
impl Target for StderrTarget {
	type Error = io::Error;

	fn level(&self) -> Level { self.level }

	fn write(&self, duration_since_start: Duration, record: &Record) -> io::Result<()> {
		let log_line = LogLine::new(duration_since_start.into(), record);
		writeln!(&mut io::stderr(), "{}", log_line)
	}

	fn flush(&self) -> io::Result<()> { io::stderr().flush() }
}
