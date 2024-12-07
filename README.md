# has-some
Implement the opposite of `is_empty` to promote better semantics for iterator 
filters (and other situations) where `!T.is_empty()` is counterintuitive, as 
well as introduce filter_friendly versions of `T::is_empty`.

Using `is_empty` in an iterator `filter` method is relatively verbose because
you can't pass the `T::is_empty` function when the iterator item is a reference,
and, anyway, you usually want to _retain_ things that are _not_ empty, a predicate 
for which you'll _always_ need a closure.

Basically, it stands that the semantics of "not is_empty" are annoying (well, to me) 
when "has some" is clearer, and even `T::is_empty` is annoying when using filters.

This crate, then, addresses those annoyances.

# Examples
It's not really rocket science, but here you go with an example where `is_empty` passed
as a function reference to an iterator filter _does_ work:

```
use has_some::HasSome

let vector = vec!["some_data".to_owned(), "".to_owned(), "more data".to_owned(), "".to_owned()];
let vector2 = vector.clone();

// If you want the empties, you can do
let empties = vector.into_iter().filter(String::is_empty).collect::<Vec<String>>();
assert_eq!(["", ""], empties.as_slice());

// If you want the non-empties, you can now do
let non_empties = vector2.into_iter().filter(String::has_some).collect::<Vec<String>>();
assert_eq!(["some_data", "more data"], non_empties.as_slice());
```

And a common example where you have `Items` that are _double_ references:

```
use has_some::HasSome

let vector = vec!["some_data", "", "more data", ""];

// If you want the empties, you can do
let empties = vector.iter().filter(str::is_empty3).collect::<Vec<&&str>>();
assert_eq!([&"", &""], empties.as_slice());

// If you want the non-empties, you can now do
let non_empties = vector2.iter().filter(str::has_some3).collect::<Vec<&&str>>();
assert_eq!([&"some_data", &"more data"], non_empties.as_slice());
```

# CHANGES
## 2.0.0
* Added the filter-friendly methods - will break custom implementations that are based on the version 1 trait
* Added an implementation for `str`
* Improved and updated documentation

## 1.0.0
Just `has_some`