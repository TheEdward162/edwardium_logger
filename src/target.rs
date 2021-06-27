use std::time::Duration;

use log::{Level, LevelFilter, Record};

/// Represents a log target.
pub trait Target {
	#[cfg(feature = "std")]
	type Error: std::error::Error;

	#[cfg(not(feature = "std"))]
	type Error: core::fmt::Display;

	/// Returns the max level this target logs.
	fn level(&self) -> Level;

	/// Whether the target wants to ignore given record.
	///
	/// This method is called before `write` to filter output.
	fn ignore(&self, _record: &Record) -> bool {
		false
	}

	/// Writes record to the target output.
	fn write(
		&self,
		duration_since_start: Duration,
		record: &Record
	) -> Result<(), Self::Error>;

	/// Flushes target output.
	fn flush(&self) -> Result<(), Self::Error>;
}

pub trait Targets {
	type Results: TargetResults;

	/// Returns the max level any of the target logs to.
	fn max_level(&self) -> LevelFilter;

	/// Writes record to the target outputs.
	fn write(
		&self,
		duration_since_start: Duration,
		record: &Record
	) -> Self::Results;

	/// Flushes target outputs.
	fn flush(&self) -> Self::Results;
}
impl<T: Target> Targets for T {
	type Results = Result<(), T::Error>;

	fn max_level(&self) -> LevelFilter {
		Target::level(self).to_level_filter()
	}

	fn write(
		&self,
		duration_since_start: Duration,
		record: &Record
	) -> Self::Results {
		if !Target::ignore(self, record) {
			Target::write(self, duration_since_start, record)
		} else {
			Ok(())
		}
	}

	fn flush(&self) -> Self::Results {
		Target::flush(self)
	}
}

pub trait TargetResults {
	// TODO: find better api?
	fn log_errors(&self, cb: impl FnMut(&dyn core::fmt::Display));
}
impl<E: core::fmt::Display> TargetResults for Result<(), E> {
	fn log_errors(&self, mut cb: impl FnMut(&dyn core::fmt::Display)) {
		match self {
			Ok(()) => (),
			Err(ref err) => cb(&err)
		}
	}
}

macro_rules! impl_for_tuple {
	(
		$(
			$gen_name: ident: $gen_num: tt
		),+
	) => {
		impl<$($gen_name: Target),+> Targets for ($($gen_name,)+) {
			type Results = (
				$(
					Result<(), $gen_name::Error>,
				)+
			);

			fn max_level(&self) -> LevelFilter {
				let max = LevelFilter::Off;

				$(
					let max = max.max(
						self.$gen_num.level().to_level_filter()
					);
				)+

				max
			}

			fn write(
				&self,
				duration_since_start: Duration,
				record: &Record
			) -> Self::Results {
				(
					$(
						if !self.$gen_num.ignore(record) {
							self.$gen_num.write(duration_since_start, record)
						} else {
							Ok(())
						},
					)+
				)
			}

			fn flush(&self) -> Self::Results {
				(
					$(
						self.$gen_num.flush(),
					)+
				)
			}
		}

		impl<$($gen_name: core::fmt::Display),+> TargetResults for ($(Result<(), $gen_name>,)+) {
			fn log_errors(&self, mut cb: impl FnMut(&dyn core::fmt::Display)) {
				$(
					match self.$gen_num {
						Ok(()) => (),
						Err(ref err) => cb(&err)
					}
				)+
			}
		}
	}
}

impl_for_tuple!(A:0);
impl_for_tuple!(A:0, B:1);
impl_for_tuple!(A:0, B:1, C:2);
impl_for_tuple!(A:0, B:1, C:2, D:3);
impl_for_tuple!(A:0, B:1, C:2, D:3, E:4);
impl_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5);
impl_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6);
impl_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7);
