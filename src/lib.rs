//! A very simple trait that relieves some tiresome things about using `is_empty` in iterator
//! filters and other "not `is_empty`" contexts.
//!
//! Three annoyances are covered:
//!
//!   * Having to always code `!T.is_empty()` when it's clearer to have `T.has_some()`;
//!   * Having to code a closure in `Iterator::filter` for the above semantics, rather
//!     than just passing a function
//!   * Having to code a closure in `Iterator::filter` even for `T.is_empty` when the `Item`
//!     of the iterator is a reference (or a double reference!).
//!
//! I fully admit that the implementation is banal - but it works!
//!
//! # Examples
//!
//! ## When you want to know if a collection has some elements
//!
//! Before:
//!
//! ```
//! let empty_string = "";
//! let not_empty_string = "this is not empty";
//! assert!(empty_string.is_empty());
//! assert!(!not_empty_string.is_empty());
//! ```
//! Now:
//!
//! ```
//! use has_some::HasSome;
//! let empty_string = "";
//! let not_empty_string = "this is not empty";
//! assert!(empty_string.is_empty());
//! assert!(not_empty_string.has_some());
//! ```
//!
//! ## When you have an iterator that emits owned types as items
//!
//! Before:
//!
//! ```
//! let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned()];
//! let vector2 = vector.clone();
//!
//! let empties: Vec<String> = vector.into_iter().filter(String::is_empty).collect();
//! let not_empties: Vec<String> = vector2.into_iter().filter(|s|!s.is_empty()).collect();
//!
//! assert_eq!([""], empties.as_slice());
//! assert_eq!(["some_data", "more data"], not_empties.as_slice());
//! ```
//!
//! Now:
//!
//! ```
//! use has_some::HasSome;
//! let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned()];
//! let vector2 = vector.clone();
//!
//! let empties: Vec<String> = vector.into_iter().filter(String::is_empty).collect();
//! let not_empties: Vec<String> = vector2.into_iter().filter(String::has_some).collect();
//!
//! assert_eq!([""], empties.as_slice());
//! assert_eq!(["some_data", "more data"], not_empties.as_slice());
//! ```
//!
//! ## When you have an iterator that emits reference types as items
//!
//! Before:
//!
//! ```
//! let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned()];
//!
//! let empties: Vec<&String> = vector.iter().filter(|s|s.is_empty()).collect();
//! let not_empties: Vec<&String> = vector.iter().filter(|s|!s.is_empty()).collect();
//!
//! assert_eq!([""], empties.as_slice());
//! assert_eq!(["some_data", "more data"], not_empties.as_slice());
//! ```
//!
//! Now:
//!
//! ```
//! use has_some::HasSome;
//! let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned()];
//!
//! let empties: Vec<&String> = vector.iter().filter(String::is_empty2).collect();
//! let not_empties: Vec<&String> = vector.iter().filter(String::has_some2).collect();
//!
//! assert_eq!([""], empties.as_slice());
//! assert_eq!(["some_data", "more data"], not_empties.as_slice());
//! ```
//!
//! Or even those pesky double references:
//!
//! ```
//! use has_some::HasSome;
//! let vector = vec!["some_data", "", "more data"];
//!
//! let empties: Vec<&&str> = vector.iter().filter(str::is_empty3).collect();
//! let not_empties: Vec<&&str> = vector.iter().filter(str::has_some3).collect();
//!
//! assert_eq!([&""], empties.as_slice());
//! assert_eq!([&"some_data", &"more data"], not_empties.as_slice());
//! ```
//!
pub trait HasSome {
    /// The opposite to `T:is_empty`.
    ///
    /// Usually implemented as `!self.is_empty()`, but as that method
    /// is not defined by a Trait in stable Rust, this needs re-implementing
    /// for each type that requires it.
    ///
    /// # Examples
    /// ```
    /// use has_some::HasSome;
    ///
    /// assert!("this is not empty".has_some());
    /// ```
    ///
    /// ```
    /// use has_some::HasSome;
    /// let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned()];
    ///
    /// let not_empties: Vec<String> = vector.into_iter().filter(String::has_some).collect();
    ///
    /// assert_eq!(["some_data","more data"], not_empties.as_slice());
    /// ```
    fn has_some(&self) -> bool;

    /// `is_empty` in a form that is suitable for use in `Iterator::filter` where
    /// `Item = &OwnedType` (e.g. `&String`)
    ///
    /// # Examples
    ///
    /// ```
    /// use has_some::HasSome;
    /// let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned()];
    ///
    /// let empties: Vec<&String> = vector.iter().filter(String::is_empty2).collect();
    ///
    /// assert_eq!([""], empties.as_slice());
    /// ```
    fn is_empty2(self: &&Self) -> bool;

    /// `is_empty` in a form that is suitable for use in `Iterator::filter` where
    /// `Item = &RefType` (e.g. `&&str`)
    ///
    /// # Examples
    ///
    /// ```
    /// use has_some::HasSome;
    /// let vector = vec!["some_data", "", "more data"];
    ///
    /// let empties: Vec<&&str> = vector.iter().filter(str::is_empty3).collect();
    ///
    /// assert_eq!([&""], empties.as_slice());
    /// ```
    fn is_empty3(self: &&&Self) -> bool {
        (*self).is_empty2()
    }

    /// `has_some` in a form that is suitable for use in `Iterator::filter` where
    /// `Item = &OwnedType` (e.g. `&String`)
    ///
    /// # Examples
    ///
    /// ```
    /// use has_some::HasSome;
    /// let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned()];
    ///
    /// let not_empties: Vec<&String> = vector.iter().filter(String::has_some2).collect();
    ///
    /// assert_eq!(["some_data","more data"], not_empties.as_slice());
    /// ```
    fn has_some2(self: &&Self) -> bool {
        (*self).has_some()
    }

    /// `has_some` in a form that is suitable for use in `Iterator::filter` where
    /// `Item = &RefType` (e.g. `&&str`)
    ///
    /// # Examples
    ///
    /// ```
    /// use has_some::HasSome;
    /// let vector = vec!["some_data", "", "more data"];
    ///
    /// let not_empties: Vec<&&str> = vector.iter().filter(str::has_some3).collect();
    ///
    /// assert_eq!([&"some_data", &"more data"], not_empties.as_slice());
    /// ```
    fn has_some3(self: &&&Self) -> bool {
        (*self).has_some()
    }
}
impl HasSome for str {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl HasSome for ::std::ffi::CStr {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl HasSome for ::std::string::String {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<Idx: PartialOrd<Idx>> HasSome for ::std::ops::Range<Idx> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<Idx: PartialOrd<Idx>> HasSome for ::std::ops::RangeInclusive<Idx> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<K, V, S: ::std::hash::BuildHasher> HasSome for ::std::collections::hash_map::HashMap<K, V, S> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<K, V> HasSome for ::std::collections::BTreeMap<K, V> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<T, S: ::std::hash::BuildHasher> HasSome for ::std::collections::hash_set::HashSet<T, S> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<T> HasSome for [T] {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<T> HasSome for ::std::collections::BinaryHeap<T> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<T> HasSome for ::std::collections::BTreeSet<T> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<T> HasSome for ::std::collections::LinkedList<T> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<T> HasSome for ::std::collections::VecDeque<T> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}
impl<T> HasSome for ::std::vec::Vec<T> {
    fn has_some(&self) -> bool {
        !self.is_empty()
    }
    fn is_empty2(self: &&Self) -> bool {
        (*self).is_empty()
    }
}

/// The rare case where `is_empty` consumes `self`.
pub trait HasSomeConsume {
    fn has_some(self) -> bool;
    fn is_empty1(&self) -> bool;
    fn has_some1(&self) -> bool;
}
impl<T> HasSomeConsume for ::std::ptr::NonNull<[T]> {
    fn has_some(self) -> bool {
        !self.is_empty()
    }
    fn is_empty1(&self) -> bool {
        (*self).is_empty()
    }
    fn has_some1(&self) -> bool {
        !(*self).is_empty()
    }
}

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
    fn test_owned_items() {
        use super::HasSome;
        let vector = vec![
            "some_data".to_owned(),
            "".to_owned(),
            "more data".to_owned(),
            "".to_owned(),
        ];
        let vector2 = vector.clone();
        // If you want the empties, get can do
        let empties = vector
            .into_iter()
            .filter(String::is_empty)
            .collect::<Vec<String>>();
        assert_eq!(["", ""], empties.as_slice());
        // But If you want the non-empties, get can do
        let non_empties = vector2
            .into_iter()
            .filter(String::has_some)
            .collect::<Vec<String>>();
        assert_eq!(["some_data", "more data"], non_empties.as_slice());
    }

    #[test]
    fn test_ref_items() {
        use super::HasSome;
        let vector = vec!["some_data", "", "more data", ""];
        let vector2 = vector.clone();
        // If you want the empties, get can do
        let empties = vector
            .into_iter()
            .filter(str::is_empty2)
            .collect::<Vec<&str>>();
        assert_eq!(["", ""], empties.as_slice());
        // But If you want the non-empties, get can do
        let non_empties = vector2
            .iter()
            .filter(str::has_some3)
            .collect::<Vec<&&str>>();
        assert_eq!([&"some_data", &"more data"], non_empties.as_slice());
    }
}
