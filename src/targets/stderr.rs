use std::{
	io::{self, Write},
	time::Duration
};

use log::{Level, Record};

use crate::target::Target;

use super::util::ignore_list::{IgnoreList, IgnoreListPatterns};

#[cfg(feature = "colored_stderr_output")]
use super::util::colored_logline::ColoredLogLine as LogLine;
#[cfg(not(feature = "colored_stderr_output"))]
use super::util::LogLine;

pub struct StderrTarget {
	level: Level,
	ignore_list: IgnoreList<'static>
}
impl StderrTarget {
	pub const fn new(level: Level, ignore_patterns: IgnoreListPatterns<'static>) -> Self {
		StderrTarget {
			level,
			ignore_list: IgnoreList::new(ignore_patterns)
		}
	}
}
impl Default for StderrTarget {
	fn default() -> Self {
		StderrTarget {
			level: log::Level::Trace,
			ignore_list: Default::default()
		}
	}
}
impl Target for StderrTarget {
	type Error = io::Error;

	fn level(&self) -> Level {
		self.level
	}

	fn ignore(&self, record: &Record) -> bool {
		self.ignore_list.ignore(record)
	}

	fn write(&self, duration_since_start: Duration, record: &Record) -> io::Result<()> {
		let log_line = LogLine::new(duration_since_start.into(), record);
		writeln!(&mut io::stderr(), "{}", log_line)
	}

	fn flush(&self) -> io::Result<()> {
		io::stderr().flush()
	}
}
