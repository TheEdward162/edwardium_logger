/// Trait used to implement custom timing in `no_std` crates where `std::time::Instant` is not available.
pub trait Timing {
	/// Returns a new reference point in time.
	fn now() -> Self where Self: Sized;

	/// Returns duration between two points in time.
	fn duration_since(&self, other: &Self) -> std::time::Duration;
}

/// Dummy `Timing` implementation that always returns zero duration.
///
/// Can be used for development or when no monotonic clock exists or is desired.
pub struct DummyTiming;
impl Timing for DummyTiming {
	fn now() -> Self {
		DummyTiming
	}

	fn duration_since(&self, _: &Self) -> std::time::Duration {
		std::time::Duration::new(0, 0)
	}
}

#[cfg(feature = "std")]
impl Timing for std::time::Instant {
	fn now() -> Self {
		std::time::Instant::now()
	}

	fn duration_since(&self, other: &Self) -> std::time::Duration {
		self.duration_since(*other)
	}
}

/* YOLO
/// Apparently we can't just crate a "zero" Instant, not even using MaybeUninit because it's not const.
/// But we can instead implement Timing for Option<Instant>, though it becomes ugly in the `duration_since` method.
/// This is an example of how such workaround might look. It's probably better not to include it in the crate.
///
/// ```
/// static mut LOGGER: edwardium_logger::Logger<
/// 	edwardium_logger::targets::stderr::StderrTarget,
/// 	[edwardium_logger::targets::stderr::StderrTarget; 1],
/// 	Option<std::time::Instant>
/// > = edwardium_logger::Logger {
/// 	targets: [
/// 		edwardium_logger::targets::stderr::StderrTarget::new(log::Level::Trace)
/// 	],
/// 	start: None,
/// 	ghost: std::marker::PhantomData
/// };
///
/// unsafe {
/// 	*LOGGER.start_mut() = Some(std::time::Instant::now());
/// 	LOGGER.init_static();
/// }
/// ```
#[cfg(feature = "std")]
impl Timing for Option<std::time::Instant> {
	fn now() -> Self {
		Some(
			std::time::Instant::now()
		)
	}

	fn duration_since(&self, other: &Self) -> std::time::Duration {
		self.as_ref().unwrap().duration_since(other.unwrap())
	}
}
*/