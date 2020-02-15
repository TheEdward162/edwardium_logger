use std::fmt::{Arguments, Display, Error, Formatter};
use std::format_args;
use std::time::Duration;

use log::{Level, Record};

pub struct Timestamp {
	minutes: u64,
	seconds: u64,
	millis: u32
}
impl Timestamp {
	pub const fn new(minutes: u64, seconds: u64, millis: u32) -> Self {
		Timestamp {
			minutes,
			seconds,
			millis
		}
	}
}
impl From<Duration> for Timestamp {
	fn from(duration: Duration) -> Self {
		let minutes = duration.as_secs() / 60;
		let seconds = duration.as_secs() % 60;
		let millis = duration.subsec_millis();

		Timestamp::new(
			minutes,
			seconds,
			millis
		)
	}
}
impl Display for Timestamp {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		f.write_fmt(
			format_args!("+{:0>3}:{:0>2}.{:0>4}", self.minutes, self.seconds, self.millis)
		)
	}
}

pub struct LogLine<'r> {
	timestamp: Timestamp,
	level: Level,
	target: &'r str,
	args: &'r Arguments<'r>
}
impl<'r> LogLine<'r> {
	pub fn new(timestamp: Timestamp, record: &'r Record<'r>) -> Self {
		LogLine {
			timestamp,
			level: record.level(),
			target: record.target(),
			args: record.args()
		}
	}
}
impl<'r> Display for LogLine<'r> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "[{}][{}] ({}) {}", self.timestamp, self.level, self.target, self.args)
	}
}