use std::time::Instant;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

mod target;
pub use target::{file::FileTarget, stdout::StdoutTarget, IgnoreList, Target};

struct Logger<T: Target + Send + Sync> {
	start_time: Instant,
	targets: Vec<T>
}
impl<T: Target + Send + Sync> Logger<T> {
	/// Creates a new Logger with given targets.
	pub fn new(targets: Vec<T>) -> Self { Logger { start_time: Instant::now(), targets } }
}
impl<T: Target + Send + Sync> Log for Logger<T> {
	fn enabled(&self, metadata: &Metadata) -> bool {
		let mut any = false;
		for target in self.targets.iter() {
			if target.level() >= metadata.level() {
				any = true;
				break
			}
		}

		any
	}

	fn log(&self, record: &Record) {
		let now = Instant::now();
		let duration_since_start = now.duration_since(self.start_time);

		for target in self.targets.iter() {
			if !target.ignore(record) {
				match target.write(record, duration_since_start) {
					Err(e) => eprintln!("Error, could not write to target: {}", e),
					Ok(_) => ()
				}
			}
		}
	}

	fn flush(&self) {
		for target in self.targets.iter() {
			match target.flush() {
				Err(e) => eprintln!("Error, could not flush target: {}", e),
				Ok(_) => ()
			}
		}
	}
}

pub fn init<T: Target + Send + Sync + 'static>(targets: Vec<T>) -> Result<(), SetLoggerError> {
	let max_level = {
		let mut max = LevelFilter::Off;

		for target in targets.iter() {
			if target.level() > max {
				max = target.level().to_level_filter();
			}
		}

		max
	};

	eprintln!("Initializing logger with max level of {:?} (static max level: {:?})", max_level, log::STATIC_MAX_LEVEL);

	let logger = Box::new(Logger::new(targets));
	log::set_boxed_logger(logger)?;

	log::set_max_level(max_level);

	Ok(())
}
