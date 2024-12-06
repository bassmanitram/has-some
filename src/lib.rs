//! A very simple trait that relieves a tiresome thing about using iterator filters and other
//! "not `is_empty`" contexts:
//! 
//! ```
//! let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned(), "".to_owned()];
//! let vector2 = vector.clone();
//! // If you want the empties, you can do
//! let empties = vector.into_iter().filter(String::is_empty).collect::<Vec<String>>();
//! assert_eq!(["", ""], empties.as_slice());
//! // But If you want the non-empties, you have to do this
//! let non_empties = vector2.into_iter().filter(|v| !v.is_empty()).collect::<Vec<String>>();
//! assert_eq!(["some_data", "more data"], non_empties.as_slice());
//! ```
//! 
//! And there are other situations where `!T.is_empty()` obfuscates things. 
//! 
//! So this brings a tiny bit more sanity to the game, allowing for:
//! 
//! ```
//! use has_some::HasSome;
//! let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned(), "".to_owned()];
//! let vector2 = vector.clone();
//! // If you want the empties, you can do
//! let empties = vector.into_iter().filter(String::is_empty).collect::<Vec<String>>();
//! assert_eq!(["", ""], empties.as_slice());
//! // Now If you want the non-empties, you can do
//! let non_empties = vector2.into_iter().filter(String::has_some).collect::<Vec<String>>();
//! assert_eq!(["some_data", "more data"], non_empties.as_slice());
//! ```
//! 
//! (Note that this doesn't help with iterators whose Item type is a reference - `is_empty`
//!  as a function reference to `filter` has never worked in those situations)
//! 
pub trait HasSome {
	fn has_some(&self) -> bool;
}

pub trait HasSomeConsume {
	fn has_some(self) -> bool;
}

impl HasSome for ::std::ffi::CStr { fn has_some(&self) -> bool { !self.is_empty() } }
impl HasSome for ::std::string::String { fn has_some(&self) -> bool { !self.is_empty() } }
impl<Idx: PartialOrd<Idx>> HasSome for ::std::ops::Range<Idx> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<Idx: PartialOrd<Idx>> HasSome for ::std::ops::RangeInclusive<Idx> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<K,V,S: ::std::hash::BuildHasher> HasSome for ::std::collections::hash_map::HashMap<K,V,S> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<K,V> HasSome for ::std::collections::BTreeMap<K,V> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T,S: ::std::hash::BuildHasher> HasSome for ::std::collections::hash_set::HashSet<T,S> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T> HasSome for [T] { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T> HasSome for ::std::collections::BinaryHeap<T> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T> HasSome for ::std::collections::BTreeSet<T> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T> HasSome for ::std::collections::LinkedList<T> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T> HasSome for ::std::collections::VecDeque<T> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T> HasSome for ::std::vec::Vec<T> { fn has_some(&self) -> bool { !self.is_empty() } }
impl<T> HasSomeConsume for ::std::ptr::NonNull<[T]> { fn has_some(self) -> bool { !self.is_empty() } }

//Used for proc macs
//impl HasSome for TokenStream { fn has_some(&self) -> bool { !self.is_empty() } }

//The issue #35428 is EIGHT years old at writing!!! - it will supersede all the above except the top two if it ever gets finalized
//impl<T: ExactSizeIterator> HasSome for T { fn has_some(&self) -> bool { !self.is_empty() } }

//The issue #76915 is FOUR years old at writing!!!
//impl<'a> HasSome for ::std::os::unix::net::SocketAncillary<'a> { fn has_some(&self) -> bool { !self.is_empty() } }

//The issue #76915 is only 11 months old years old at writing, though the last comment was 8 months ago
//impl HasSome for ::std::ffi::os_str::OsStr { fn has_some(&self) -> bool { !self.is_empty() } }

#[cfg(test)]
mod tests {
	#[test]
	fn test() {
		use super::HasSome;
		let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned(), "".to_owned()];
		let vector2 = vector.clone();
		// If you want the empties, get can do
		let empties = vector.into_iter().filter(String::is_empty).collect::<Vec<String>>();
		assert_eq!(["", ""], empties.as_slice());
		// But If you want the non-empties, get can do
		let non_empties = vector2.into_iter().filter(String::has_some).collect::<Vec<String>>();
		assert_eq!(["some_data", "more data"], non_empties.as_slice());
	}
}
