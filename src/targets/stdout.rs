use std::{io, io::Write, time::Duration};

use log::{Level, Record};

use crate::target::Target;

#[cfg(feature = "colored_stdout_output")]
use super::util::colored_logline::ColoredLogLine as LogLine;

#[cfg(not(feature = "colored_stdout_output"))]
use super::util::LogLine;

pub struct StdoutTarget {
	level: Level
}
impl StdoutTarget {
	pub const fn new(level: Level) -> Self { StdoutTarget { level } }
}
impl Default for StdoutTarget {
	fn default() -> Self { StdoutTarget { level: log::Level::Trace } }
}
impl Target for StdoutTarget {
	type Error = io::Error;

	fn level(&self) -> Level { self.level }

	fn write(&self, duration_since_start: Duration, record: &Record) -> io::Result<()> {
		let log_line = LogLine::new(duration_since_start.into(), record);
		writeln!(&mut io::stdout(), "{}", log_line)
	}

	fn flush(&self) -> io::Result<()> { io::stdout().flush() }
}
