use log::{ Level, Record };

use std::io::{ self, Write };

use super::{ Target, IgnoreList };

pub struct StdoutTarget {
	level: Level,
	ignore_list: IgnoreList
}
impl StdoutTarget {
	pub fn new(level: Level, ignore_list: IgnoreList) -> Self {
		StdoutTarget {
			level,
			ignore_list
		}
	}
}
impl Default for StdoutTarget {
	fn default() -> Self {
		StdoutTarget {
			level: log::Level::Trace,
			ignore_list: Default::default()
		}
	}
}
impl Target for StdoutTarget {
	fn level(&self) -> Level {
		self.level
	}

	fn ignore(&self, record: &Record) -> bool {
		self.ignore_list.ignore(record)
	}

	fn write(&self, string: &str) -> io::Result<()> {
		writeln!(&mut io::stdout(), "{}", string)
	}

	fn flush(&self) -> io::Result<()> {
		io::stdout().flush()
	}
}
