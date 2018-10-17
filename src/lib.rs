//! Create a [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) object
//! that gets its data incrementally from a function.
//!
//! This lets you read from an a vector of vectors or create
//! a reader that gets blocks from a database or other data source.
//!
//! Example:
//!
//! ```rust
//! let many_strings = ["one", "two", "three"];
//! let mut pos = 0;
//! std::io::copy(
//!     &mut read_with::ReadWith::new(
//!         ||
//!         {
//!             if pos == many_strings.len() { return None; }
//!             let o = many_strings[pos];
//!             pos+=1;
//!             Some(o)
//!         }
//!     ),
//!     &mut std::io::stdout(),
//! ).unwrap();
//! ```

use std::io::Read;

/// An object that implements the `Read` trait
pub struct ReadWith<F, S>
	where F: FnMut() -> Option<S>,
	S: AsRef<[u8]> + Default
{
	f: F,
	current: S,
	offset: usize,
	end: bool
}

impl<F, S> ReadWith<F, S>
	where F: FnMut() -> Option<S>,
	S: AsRef<[u8]> + Default
{
	/// Create an object that will read from the given function.
	///
	/// Keeps on reading from `f` until it returns a None.
	/// The function may return anything that can be turned into
	/// a `&[u8]` which includes `String` and `&str`.
	pub fn new(f: F) -> Self
	{
		ReadWith
		{
			f: f,
			current: Default::default(),
			offset: 0,
			end: false,
		}
	}
}

impl<F,S> Read for ReadWith<F, S>
	where F: FnMut() -> Option<S>,
	S: AsRef<[u8]> + Default
{
	fn read(&mut self, buf: &mut [u8])
		-> std::io::Result<usize>
	{
		let mut wrote = 0;
		while !self.end && wrote < buf.len()
		{
			let count = (buf.len()-wrote).min(self.current.as_ref().len()-self.offset);
			buf[wrote..wrote+count]
				.copy_from_slice( &self.current.as_ref()[self.offset..self.offset+count] );
			wrote += count;
			self.offset += count;
			if self.offset == self.current.as_ref().len()
			{
				self.offset = 0;
				let n = (self.f)();
				if let Some(n) = n
					{ self.current = n; }
				else
					{ self.end = true; }
			}
		}

		Ok(wrote)
	}
}


#[cfg(test)]
mod tests
{
	use ::ReadWith;

	#[test]
	fn references()
	{
		let mut output = vec!();
		let many_strings = ["one", "two", "three"];
		let mut pos = 0;

		::std::io::copy(
			&mut ReadWith::new(
				||
				{
					if pos == many_strings.len() { return None; }
					let o = many_strings[pos];
					pos+=1;
					Some(o)
				}
			),
			&mut output,
		).unwrap();
		assert_eq!("onetwothree", ::std::str::from_utf8(&output).unwrap());
	}

	#[test]
	fn strings()
	{
		let mut output = vec!();
		let many_strings = ["one", "two", "three"];
		let mut pos = 0;
		::std::io::copy(
			&mut ReadWith::new(
				||
				{
					if pos == many_strings.len() { return None; }
					let o = many_strings[pos];
					pos+=1;
					Some(o.to_string() + "\n")
				}
			),
			&mut output,
		).unwrap();
		assert_eq!("one\ntwo\nthree\n", ::std::str::from_utf8(&output).unwrap());
	}
}
