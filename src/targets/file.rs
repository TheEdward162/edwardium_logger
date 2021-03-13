use std::{
	fs::{File, OpenOptions},
	io,
	io::Write,
	path::Path,
	sync::Mutex,
	time::Duration
};

use log::{Level, Record};

use crate::target::Target;

use super::util::ignore_list::{IgnoreList, IgnoreListPatterns};

pub struct FileTarget {
	level: Level,
	ignore_list: IgnoreList<'static>,
	file: Mutex<File>
}
impl FileTarget {
	pub fn new(
		level: Level,
		path: &Path,
		ignore_patterns: IgnoreListPatterns<'static>
	) -> io::Result<Self> {
		let file = Mutex::new(
			OpenOptions::new()
				.write(true)
				.append(true)
				.create(true)
				.open(path)?
		);

		Ok(FileTarget {
			level,
			ignore_list: IgnoreList::new(ignore_patterns),
			file
		})
	}
}
impl Target for FileTarget {
	type Error = io::Error;

	fn level(&self) -> Level {
		self.level
	}

	fn ignore(&self, record: &Record) -> bool {
		self.ignore_list.ignore(record)
	}

	fn write(
		&self,
		duration_since_start: Duration,
		record: &Record
	) -> Result<(), Self::Error> {
		match self.file.lock() {
			Err(_) => Err(io::Error::new(
				io::ErrorKind::Other,
				"mutex poison error"
			)),
			Ok(mut lock) => {
				let log_line = super::util::LogLine::new(
					duration_since_start.into(),
					record
				);
				writeln!(&mut lock, "{}", log_line)
			}
		}
	}

	fn flush(&self) -> Result<(), Self::Error> {
		match self.file.lock() {
			Err(_) => Err(io::Error::new(
				io::ErrorKind::Other,
				"mutex poison error"
			)),
			Ok(mut lock) => lock.flush()
		}
	}
}
