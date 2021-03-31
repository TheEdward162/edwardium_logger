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
	/// Returns the max level any of the target logs to.
	fn max_level(&self) -> LevelFilter;

	/// Writes record to the target outputs.
	///
	/// Calls `error_cb` for each error.
	fn write(
		&self,
		duration_since_start: Duration,
		record: &Record,
		error_cb: &dyn Fn(&dyn core::fmt::Display)
	);

	/// Flushes target outputs.
	///
	/// Calls `error_cb` for each error.
	fn flush(&self, error_cb: &dyn Fn(&dyn core::fmt::Display));
}
impl<T: Target> Targets for T {
	fn max_level(&self) -> LevelFilter {
		Target::level(self).to_level_filter()
	}

	fn write(
		&self,
		duration_since_start: Duration,
		record: &Record,
		error_cb: &dyn Fn(&dyn core::fmt::Display)
	) {
		if !Target::ignore(self, record) {
			match Target::write(self, duration_since_start, record) {
				Ok(_) => (),
				Err(err) => error_cb(&err)
			}
		}
	}

	fn flush(&self, error_cb: &dyn Fn(&dyn core::fmt::Display)) {
		match Target::flush(self) {
			Ok(_) => (),
			Err(err) => error_cb(&err)
		}
	}
}

macro_rules! impl_targets_for_tuple {
	(
		$(
			$gen_name: ident: $gen_num: tt
		),+
	) => {
		impl<$($gen_name: Target),+> Targets for ($($gen_name,)+) {
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
				record: &Record,
				error_cb: &dyn Fn(&dyn core::fmt::Display)
			) {
				$(
					if !self.$gen_num.ignore(record) {
						match self.$gen_num.write(
							duration_since_start,
							record
						) {
							Ok(()) => (),
							Err(err) => { error_cb(&err); }
						}
					}
				)+
			}

			fn flush(&self, error_cb: &dyn Fn(&dyn core::fmt::Display)) {
				$(
					match self.$gen_num.flush() {
						Ok(()) => (),
						Err(err) => { error_cb(&err); }
					}
				)+
			}
		}
	}
}

impl_targets_for_tuple!(A:0);
impl_targets_for_tuple!(A:0, B:1);
impl_targets_for_tuple!(A:0, B:1, C:2);
impl_targets_for_tuple!(A:0, B:1, C:2, D:3);
impl_targets_for_tuple!(A:0, B:1, C:2, D:3, E:4);
impl_targets_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5);
impl_targets_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6);
impl_targets_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7);
