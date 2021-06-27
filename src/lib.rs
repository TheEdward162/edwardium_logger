//! Simple logger implementation that works using targets.
//!
//! Instead of needing multiple logging crates to log to multiple targets
//! this crate provides a layer of abstraction and a few default targets.
//! A target has to implement the [`Target`](target/trait.Target.html) trait.
//!
//! To start logging, create the [`Logger`](struct.Logger.html) object either statically or dynamically
//! and then call one of its `init_` methods.
//!
//! For example, for a dynamic logger (requires std feature):
//!
//! ```
//! use edwardium_logger::targets::stderr::StderrTarget;
//! let logger = edwardium_logger::Logger::new(
//! 	StderrTarget::new(log::Level::Trace, Default::default()),
//! 	std::time::Instant::now()
//! );
//! logger.init_boxed().expect("Could not initialize logger");
//! ```
//!
//! Logger can also be created and set statically, though this has a few caveats (read the documentation of [`Logger::new`](struct.Logger.html#method.new) for more):
//!
//! ```
//! use edwardium_logger::{
//! 	targets::{
//! 		stderr::StderrTarget,
//! 		util::ignore_list::IgnoreList
//! 	},
//! 	timing::DummyTiming
//! };
//! static LOGGER: edwardium_logger::Logger<
//! 	(StderrTarget),
//! 	DummyTiming
//! > = edwardium_logger::Logger {
//! 	targets: StderrTarget::new(log::Level::Trace, IgnoreList::EMPTY_PATTERNS),
//! 	start: DummyTiming
//! };
//! LOGGER.init_static();
//! ```

// TODO: Not really tested
// This crate also has a `no_std` (disable default features) version and
// UART logging target using [`embedded-serial`](https://docs.rs/embedded-serial) is provided
// under the `uart_target` feature.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use core as std;

use log::{Log, Metadata, Record, SetLoggerError};

pub mod target;
pub mod timing;

pub mod targets;

use target::TargetResults;

/// Logger
///
/// TODO: The fields of this struct are public only because generics on const fns are unstable.
pub struct Logger<Targ, Time>
where
	Targ: target::Targets + Send + Sync + 'static,
	Time: timing::Timing + Send + Sync + 'static
{
	pub targets: Targ,
	pub start: Time
}
impl<Targ, Time> Logger<Targ, Time>
where
	Targ: target::Targets + Send + Sync + 'static,
	Time: timing::Timing + Send + Sync + 'static
{
	/// Creates a new Logger.
	///
	/// TODO: This function should be `const` but that is unstable with generic parameters, thus the fields of the logger are made public instead.
	pub fn new(targets: Targ, start: Time) -> Self {
		Logger { targets, start }
	}

	/// Returns a reference to the `start` field.
	pub fn start(&self) -> &Time {
		&self.start
	}

	/// Returns a mutable reference to the `start` field.
	///
	/// This can be used to have a `static mut` logger and change the `start` at the beginning of the program.
	pub fn start_mut(&mut self) -> &mut Time {
		&mut self.start
	}

	#[cfg(feature = "std")]
	pub fn init_boxed(self) -> Result<(), SetLoggerError> {
		let max_level = self.targets.max_level();
		eprintln!("Initializing logger with max level of {:?} (static max level: {:?})", max_level, log::STATIC_MAX_LEVEL);
		log::set_max_level(max_level);

		let logger = Box::new(self);
		log::set_boxed_logger(logger)?;
		Ok(())
	}

	// TODO: On thumbv6 this won't compile
	pub fn init_static(&'static self) -> Result<(), SetLoggerError> {
		let max_level = self.targets.max_level();
		#[cfg(feature = "std")]
		eprintln!("Initializing logger with max level of {:?} (static max level: {:?})", max_level, log::STATIC_MAX_LEVEL);
		log::set_max_level(max_level);

		log::set_logger(self)?;
		Ok(())
	}

	pub unsafe fn init_static_racy(
		&'static self
	) -> Result<(), SetLoggerError> {
		let max_level = self.targets.max_level();
		#[cfg(feature = "std")]
		eprintln!("Initializing logger with max level of {:?} (static max level: {:?})", max_level, log::STATIC_MAX_LEVEL);
		log::set_max_level(max_level);

		log::set_logger_racy(self)?;
		Ok(())
	}

	#[cfg(feature = "std")]
	fn on_error(&self, error: &dyn std::fmt::Display) {
		eprintln!("{}", error);
	}

	#[cfg(not(feature = "std"))]
	fn on_error(&self, error: &dyn core::fmt::Display) {
		// Nothing to do?
	}
}
impl<Targ, Time> Log for Logger<Targ, Time>
where
	Targ: target::Targets + Send + Sync + 'static,
	Time: timing::Timing + Send + Sync + 'static
{
	fn enabled(&self, metadata: &Metadata) -> bool {
		self.targets.max_level() >= metadata.level()
	}

	fn log(&self, record: &Record) {
		let now = Time::now();
		let duration_since_start = now.duration_since(&self.start);

		let results = self.targets.write(duration_since_start, record);
		results.log_errors(
			|err| self.on_error(err)
		);
	}

	fn flush(&self) {
		let results = self.targets.flush();
		results.log_errors(
			|err| self.on_error(err)
		);
	}
}
