use std::{
	fs::{File, OpenOptions},
	io::{self, Write},
	path::Path,
	sync::Mutex,
	time::Duration
};

use log::{Level, Record};

use super::{IgnoreList, Target};

pub struct FileTarget {
	level: Level,
	ignore_list: IgnoreList,

	file: Mutex<File>
}
impl FileTarget {
	pub fn new(level: Level, ignore_list: IgnoreList, path: &Path) -> io::Result<Self> {
		let file =
			Mutex::new(OpenOptions::new().write(true).truncate(true).create(true).open(path)?);


		Ok(FileTarget { level, ignore_list, file })
	}
}
impl Target for FileTarget {
	fn level(&self) -> Level { self.level }

	fn ignore(&self, record: &Record) -> bool { self.ignore_list.ignore(record) }

	fn write(&self, record: &Record, duration_since_start: Duration) -> io::Result<()> {
		match self.file.lock() {
			Err(_) => Err(io::Error::new(io::ErrorKind::Other, "mutex poison error")),
			Ok(mut lock) => {
				let string = crate::target::create_log_line(record, duration_since_start);
				writeln!(&mut lock, "{}", string)
			}
		}
	}

	fn flush(&self) -> io::Result<()> {
		match self.file.lock() {
			Err(_) => Err(io::Error::new(io::ErrorKind::Other, "mutex poison error")),
			Ok(mut lock) => lock.flush()
		}
	}
}
