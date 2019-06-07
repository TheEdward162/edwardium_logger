use log::{ Level, Record };

use std::path::Path;
use std::fs::{ File, OpenOptions };
use std::io::{ self, Write };
use std::sync::Mutex;

use super::{ Target, IgnoreList };

pub struct FileTarget {
	level: Level,
	ignore_list: IgnoreList,

	file: Mutex<File>
}
impl FileTarget {
	pub fn new(level: Level, ignore_list: IgnoreList, path: &Path) -> io::Result<Self> {
		let file = Mutex::new(
			OpenOptions::new()
				.write(true)
				.truncate(true)
				.create(true)
			.open(path)?
		);


		Ok(FileTarget {
			level,
			ignore_list,

			file
		})
	}
}
impl Target for FileTarget {
	fn level(&self) -> Level {
		self.level
	}

	fn ignore(&self, record: &Record) -> bool {
		self.ignore_list.ignore(record)
	}
	
	fn write(&self, string: &str) -> io::Result<()> {
		match self.file.lock() {
			Err(_) => {
				Err(io::Error::new(io::ErrorKind::Other, "mutex poison error"))
			},
			Ok(mut lock) => writeln!(&mut lock, "{}", string)
		}
	}

	fn flush(&self) -> io::Result<()> {
		match self.file.lock() {
			Err(_) => {
				Err(io::Error::new(io::ErrorKind::Other, "mutex poison error"))
			},
			Ok(mut lock) => lock.flush()
		}
	}
}