use std::{
	io::{self, Write},
	time::Duration
};

use log::{Level, Record};

use super::{IgnoreList, Target};

pub struct StdoutTarget {
	level: Level,
	ignore_list: IgnoreList
}
impl StdoutTarget {
	pub fn new(level: Level, ignore_list: IgnoreList) -> Self {
		StdoutTarget { level, ignore_list }
	}
}
impl Default for StdoutTarget {
	fn default() -> Self {
		StdoutTarget { level: log::Level::Trace, ignore_list: Default::default() }
	}
}
impl Target for StdoutTarget {
	fn level(&self) -> Level { self.level }

	fn ignore(&self, record: &Record) -> bool { self.ignore_list.ignore(record) }

	fn write(&self, record: &Record, duration_since_start: Duration) -> io::Result<()> {
		let string = crate::target::create_log_line(record, duration_since_start);
		writeln!(&mut io::stdout(), "{}", string)
	}

	fn flush(&self) -> io::Result<()> { io::stdout().flush() }
}
