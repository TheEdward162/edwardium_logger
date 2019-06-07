use log::{ Log, Record, Metadata, SetLoggerError, LevelFilter };

use std::time::Instant;

mod target;
pub use target::{ Target, IgnoreList, stdout::StdoutTarget, file::FileTarget };

struct Logger {
	start_time: Instant,

	stdout_target: Option<StdoutTarget>,
	file_target: Option<FileTarget>
}
impl Logger {
	pub fn new(stdout_target: Option<StdoutTarget>, file_target: Option<FileTarget>) -> Self {
		Logger {
			start_time: Instant::now(),
			
			stdout_target,
			file_target
		}
	}

	fn create_timestamp(&self) -> String {
		let now = Instant::now();
		let duration_since_start = now.duration_since(self.start_time);

		let minutes = duration_since_start.as_secs() / 60;
		let seconds = duration_since_start.as_secs() % 60;
		let millis = duration_since_start.subsec_millis();

		format!("+{:0>3}:{:0>2}.{:0>4}", minutes, seconds, millis)
	}

	fn create_log_line(stamp: &str, record: &Record) -> String {
		format!("[{}][{}] ({}) {}", stamp, record.level(), record.target(), record.args())
	}
}
impl Log for Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		macro_rules! check_enabled {
			($id: ident) => {
				match self.$id.as_ref() {
					None => false,
					Some(t) => metadata.level() <= t.level()
				}
			}
		}
		
		check_enabled!(stdout_target) || check_enabled!(file_target)
    }

	fn log(&self, record: &Record) {
		let stamp = self.create_timestamp();
		let log_string = Logger::create_log_line(&stamp, record);

		macro_rules! check_log {
			($id: ident) => {
				match self.$id.as_ref() {
					None => (),
					Some(t) => if !t.ignore(record) {
						match t.write(&log_string) {
							Err(e) => eprintln!("Error, could not write to target: {}", e),
							Ok(_) => ()
						}
					}
				}
			}
		}

		check_log!(stdout_target);
		check_log!(file_target);
	}

	fn flush(&self) {
		macro_rules! check_flush {
			($id: ident) => {
				match self.$id.as_ref() {
					None => (),
					Some(t) => match t.flush() {
						Err(e) => eprintln!("Error, could not flush target: {}", e),
						Ok(_) => ()
					}
				}
			}
		}

		check_flush!(stdout_target);
		check_flush!(file_target);
	}
}

pub fn init(stdout: Option<StdoutTarget>, file: Option<FileTarget>) -> Result<(), SetLoggerError> {
	let max_level = {
		let mut max = LevelFilter::Off;

		if let Some(ref stdout) = stdout {
			if stdout.level() > max {
				max = stdout.level().to_level_filter();
			}
		}
		if let Some(ref file) = file {
			if file.level() > max {
				max = file.level().to_level_filter();
			}
		}

		max
	};

	let logger = Box::new(Logger::new(stdout, file));
	log::set_boxed_logger(logger)?;
	log::set_max_level(max_level);
	Ok(())
}