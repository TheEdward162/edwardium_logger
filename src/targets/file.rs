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

pub struct FileTarget {
	level: Level,
	file: Mutex<File>
}
impl FileTarget {
	pub fn new(level: Level, path: &Path) -> io::Result<Self> {
		let file = Mutex::new(OpenOptions::new().write(true).append(true).create(true).open(path)?);

		Ok(FileTarget { level, file })
	}
}
impl Target for FileTarget {
	type Error = io::Error;

	fn level(&self) -> Level { self.level }

	fn write(&self, duration_since_start: Duration, record: &Record) -> Result<(), Self::Error> {
		match self.file.lock() {
			Err(_) => Err(io::Error::new(io::ErrorKind::Other, "mutex poison error")),
			Ok(mut lock) => {
				let log_line = super::util::LogLine::new(duration_since_start.into(), record);
				writeln!(&mut lock, "{}", log_line)
			}
		}
	}

	fn flush(&self) -> Result<(), Self::Error> {
		match self.file.lock() {
			Err(_) => Err(io::Error::new(io::ErrorKind::Other, "mutex poison error")),
			Ok(mut lock) => lock.flush()
		}
	}
}
