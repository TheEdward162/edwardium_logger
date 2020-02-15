use std::fmt::{Error, Write};
use std::sync::Mutex;
use std::time::Duration;

use embedded_serial::{ImmutBlockingTx, ImmutBlockingTxWithTimeout, ImmutNonBlockingTx, MutBlockingTx, MutBlockingTxWithTimeout, MutNonBlockingTx};
use log::{Level, Record};

use crate::target::Target;

/// Marker trait to avoid repetition.
pub trait WritableTx: Write {
	/// Type that will be wrapped.
	type Type;
	/// Data type that will be stored in the `data` field of the wrapper.
	///
	/// Can be used to pass and store additional data.
	type Data;

	fn new(sink: Self::Type, data: Self::Data) -> Self where Self: Sized;
}

macro_rules! impl_writable_tx_wrap {
	(
		struct $wrap_name: ident<$trait_name: ident> {
			sink,
			data: $data_type: ty
		}
		impl Write {
			$( $write_impl: tt )+
		}
	) => {
		struct $wrap_name<T: $trait_name> {
			sink: T,
			#[allow(dead_code)]
			data: $data_type
		}
		impl<T: $trait_name> Write for $wrap_name<T> {
			$( $write_impl )+
		}
		impl<T: $trait_name> WritableTx for $wrap_name<T> {
			type Type = T;
			type Data = $data_type;

			fn new(sink: Self::Type, data: Self::Data) -> Self {
				$wrap_name {
					sink,
					data
				}
			}
		}
	}
}

impl_writable_tx_wrap! {
	struct ImmutBlockingTxWrap<ImmutBlockingTx> {
		sink,
		data: ()
	}
	impl Write {
		fn write_str(&mut self, s: &str) -> Result<(), Error> {
			match self.sink.puts(s.as_bytes()) {
				Ok(()) => Ok(()),
				Err((_len, _err)) => Err(std::fmt::Error)
			}
		}
	}
}
impl_writable_tx_wrap! {
	struct ImmutBlockingTxWithTimeoutWrap<ImmutBlockingTxWithTimeout> {
		sink,
		data: T::Timeout
	}
	impl Write {
		fn write_str(&mut self, s: &str) -> Result<(), Error> {
			match self.sink.puts_wait(s.as_bytes(), &self.data) {
				Ok(_len) => Ok(()),
				Err((_len, _err)) => Err(std::fmt::Error)
			}
		}
	}
}
impl_writable_tx_wrap! {
	struct ImmutNonBlockingTxWrap<ImmutNonBlockingTx> {
		sink,
		data: u32 // How many times to reattempt to send the message as a whole.
	}
//	impl Write {
//		fn write_str(&mut self, s: &str) -> Result<(), Error> {
//			for byte in s.bytes() {
//				let mut attempts = self.data + 1;
//				for _ in 0 .. attempts {
//					match self.sink.putc_try(byte) {
//						Ok(Some(_)) => {
//							attempts = 0;
//							break
//						},
//						Ok(None) => continue,
//						Err(_err) => return Err(std::fmt::Error)
//					}
//				}
//
//				if attempts != 0 {
//					return Err(std::fmt::Error)
//				}
//			}
//
//			Ok(())
//		}
//	}
	impl Write {
		fn write_str(&mut self, s: &str) -> Result<(), Error> {
			let mut start = 0;

			for _ in 0 .. self.data + 1 {
				match self.sink.puts_try(&s.as_bytes()[start ..]) {
					Ok(len) if start + len == s.as_bytes().len() => {
						return Ok(())
					}
					Ok(len) => {
						start += len;
						continue
					}
					Err((_len, _err)) => return Err(std::fmt::Error)
				}
			}

			Err(std::fmt::Error)
		}
	}
}

impl_writable_tx_wrap! {
	struct MutBlockingTxWrap<MutBlockingTx> {
		sink,
		data: ()
	}
	impl Write {
		fn write_str(&mut self, s: &str) -> Result<(), Error> {
			match self.sink.puts(s.as_bytes()) {
				Ok(()) => Ok(()),
				Err((_len, _err)) => Err(std::fmt::Error)
			}
		}
	}
}
impl_writable_tx_wrap! {
	struct MutBlockingTxWithTimeoutWrap<MutBlockingTxWithTimeout> {
		sink,
		data: T::Timeout
	}
	impl Write {
		fn write_str(&mut self, s: &str) -> Result<(), Error> {
			match self.sink.puts_wait(s.as_bytes(), &self.data) {
				Ok(_len) => Ok(()),
				Err((_len, _err)) => Err(std::fmt::Error)
			}
		}
	}
}
impl_writable_tx_wrap! {
	struct MutNonBlockingTxWrap<MutNonBlockingTx> {
		sink,
		data: u32 // How many times to reattempt to send the message as a whole.
	}
//	impl Write {
//		fn write_str(&mut self, s: &str) -> Result<(), Error> {
//			for byte in s.bytes() {
//				let mut attempts = self.data + 1;
//				for _ in 0 .. attempts {
//					match self.sink.putc_try(byte) {
//						Ok(Some(_)) => {
//							attempts = 0;
//							break
//						},
//						Ok(None) => continue,
//						Err(_err) => return Err(std::fmt::Error)
//					}
//				}
//
//				if attempts != 0 {
//					return Err(std::fmt::Error)
//				}
//			}
//
//			Ok(())
//		}
//	}
	impl Write {
		fn write_str(&mut self, s: &str) -> Result<(), Error> {
			let mut start = 0;

			for _ in 0 .. self.data + 1 {
				match self.sink.puts_try(&s.as_bytes()[start ..]) {
					Ok(len) if start + len == s.as_bytes().len() => {
						return Ok(())
					}
					Ok(len) => {
						start += len;
						continue
					}
					Err((_len, _err)) => return Err(std::fmt::Error)
				}
			}

			Err(std::fmt::Error)
		}
	}
}

pub struct UartTarget<T: WritableTx> {
	level: Level,
	sink: Mutex<T>,
}
impl<T: WritableTx> UartTarget<T> {
	pub fn new(
		level: Level,
		sink: T::Type,
		config: T::Data
	) -> Self {
		UartTarget {
			level,
			sink: Mutex::new(
				T::new(sink, config)
			),
		}
	}
}
impl<T: WritableTx> Target for UartTarget<T> {
	type Error = Error;
	// TODO: fmt::Formatter is currently designed such that it's not possible to propagate io errors back to the caller

	fn level(&self) -> Level {
		self.level
	}

	fn write(&self, duration_since_start: Duration, record: &Record) -> Result<(), Self::Error> {
		let log_line = super::util::LogLine::new(
			duration_since_start.into(),
			record,
		);

		let mut lock = self.sink.lock().unwrap();
		writeln!(&mut lock, "{}", log_line)
	}

	fn flush(&self) -> Result<(), Self::Error> {
		// TODO: Is there any other way?
		Ok(())
	}
}