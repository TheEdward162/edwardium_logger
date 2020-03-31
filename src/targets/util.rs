use std::{
	fmt::{Arguments, Display, Error, Formatter},
	time::Duration
};

use log::{Level, Record};

pub struct Timestamp {
	minutes: u64,
	seconds: u64,
	millis: u32
}
impl Timestamp {
	pub const fn new(minutes: u64, seconds: u64, millis: u32) -> Self {
		Timestamp { minutes, seconds, millis }
	}
}
impl From<Duration> for Timestamp {
	fn from(duration: Duration) -> Self {
		let minutes = duration.as_secs() / 60;
		let seconds = duration.as_secs() % 60;
		let millis = duration.subsec_millis();

		Timestamp::new(minutes, seconds, millis)
	}
}
impl Display for Timestamp {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "+{:0>3}:{:0>2}.{:0>4}", self.minutes, self.seconds, self.millis)
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
		LogLine { timestamp, level: record.level(), target: record.target(), args: record.args() }
	}
}
impl<'r> Display for LogLine<'r> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "[{}][{}] ({}) {}", self.timestamp, self.level, self.target, self.args)
	}
}

#[cfg(feature = "colored_logline")]
pub mod colored_logline {
	use std::fmt::{Arguments, Display, Error, Formatter};

	use log::{Level, Record};

	use termion::color::{self, Fg};

	use super::Timestamp;

	pub struct ColoredLogLine<'r> {
		timestamp: Timestamp,
		level: Level,
		target: &'r str,
		args: &'r Arguments<'r>
	}
	impl<'r> ColoredLogLine<'r> {
		pub fn new(timestamp: Timestamp, record: &'r Record<'r>) -> Self {
			ColoredLogLine {
				timestamp,
				level: record.level(),
				target: record.target(),
				args: record.args()
			}
		}
	}
	impl<'r> Display for ColoredLogLine<'r> {
		fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
			#[derive(Debug)]
			struct ApiDesignIsHard(Level);
			impl termion::color::Color for ApiDesignIsHard {
				fn write_fg(&self, f: &mut Formatter) -> Result<(), Error> {
					match self.0 {
						Level::Error => color::Red.write_fg(f),
						Level::Warn => color::Magenta.write_fg(f),
						Level::Info => color::Green.write_fg(f),
						Level::Debug => color::Blue.write_fg(f),
						Level::Trace => color::Black.write_fg(f)
					}
				}

				fn write_bg(&self, _: &mut Formatter) -> Result<(), Error> { unimplemented!() }
			}

			write!(
				f,
				"[{}{}{}][{}{}{}] ({}{}{}) {}",
				Fg(color::Yellow),
				self.timestamp,
				Fg(color::Reset),
				Fg(ApiDesignIsHard(self.level)),
				self.level,
				Fg(color::Reset),
				Fg(color::Cyan),
				self.target,
				Fg(color::Reset),
				self.args
			)
		}
	}
}
