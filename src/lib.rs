#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use core as std;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

pub mod timing;
pub mod target;

pub mod targets;

/// Logger
///
/// TODO: The fields of this struct are public only because generics on const fns are unstable.
pub struct Logger<L, C, T> where
	L: target::Target + Send + Sync + 'static,
	C: AsRef<[L]> + Send + Sync + 'static,
	T: timing::Timing + Send + Sync + 'static
{
	pub targets: C,
	pub start: T,
	pub ghost: std::marker::PhantomData<L>
}
impl<L, C, T> Logger<L, C, T> where
	L: target::Target + Send + Sync + 'static,
	C: AsRef<[L]> + Send + Sync + 'static,
	T: timing::Timing + Send + Sync + 'static
{
	/// Creates a new Logger.
	///
	/// TODO: This function should be `const` but that is unstable with generic parameters, thus the fields of the logger are made public instead.
	pub fn new(targets: C, start: T) -> Self {
		Logger {
			targets,
			start,
			ghost: std::marker::PhantomData,
		}
	}

	/// Returns a reference to the `start` field.
	pub fn start(&self) -> &T {
		&self.start
	}

	/// Returns a mutable reference to the `start` field.
	///
	/// This can be used to have a `static mut` logger and change the `start` at the beginning of the program.
	pub fn start_mut(&mut self) -> &mut T {
		&mut self.start
	}

	fn targets_max_level(&self) -> LevelFilter {
		self.targets.as_ref().iter().fold(
			LevelFilter::Off, |max, target| {
				if target.level() > max {
					target.level().to_level_filter()
				} else {
					max
				}
			},
		)
	}

	#[cfg(feature = "std")]
	pub fn init_boxed(self) -> Result<(), SetLoggerError> {
		let max_level = self.targets_max_level();
		eprintln!("Initializing logger with max level of {:?} (static max level: {:?})", max_level, log::STATIC_MAX_LEVEL);
		log::set_max_level(max_level);

		let logger = Box::new(self);
		log::set_boxed_logger(logger)?;
		Ok(())
	}

	// TODO: On thumbv6 this won't compile
	pub fn init_static(&'static self) -> Result<(), SetLoggerError> {
		let max_level = self.targets_max_level();
		#[cfg(feature = "std")]
		eprintln!("Initializing logger with max level of {:?} (static max level: {:?})", max_level, log::STATIC_MAX_LEVEL);
		log::set_max_level(max_level);

		log::set_logger(self)?;
		Ok(())
	}

	pub unsafe fn init_static_racy(&'static self) -> Result<(), SetLoggerError> {
		let max_level = self.targets_max_level();
		#[cfg(feature = "std")]
		eprintln!("Initializing logger with max level of {:?} (static max level: {:?})", max_level, log::STATIC_MAX_LEVEL);
		log::set_max_level(max_level);

		log::set_logger_racy(self)?;
		Ok(())
	}

	#[cfg(feature = "std")]
	fn on_error(&self, error: L::Error) {
		eprintln!("{}", error);
	}

	#[cfg(not(feature = "std"))]
	fn on_error(&self, error: L::Error) {
		// Nothing to do
	}
}
impl<L, C, T> Log for Logger<L, C, T> where
	L: target::Target + Send + Sync,
	C: AsRef<[L]> + Send + Sync,
	T: timing::Timing + Send + Sync
{
	fn enabled(&self, metadata: &Metadata) -> bool {
		self.targets.as_ref().iter().any(|target| target.level() >= metadata.level())
	}

	fn log(&self, record: &Record) {
		let now = T::now();
		let duration_since_start = now.duration_since(&self.start);

		for target in self.targets.as_ref().iter() {
			if !target.ignore(record) {
				match target.write(duration_since_start, record) {
					Err(e) => self.on_error(e),
					Ok(_) => ()
				}
			}
		}
	}

	fn flush(&self) {
		for target in self.targets.as_ref().iter() {
			match target.flush() {
				Err(e) => self.on_error(e),
				Ok(_) => ()
			}
		}
	}
}