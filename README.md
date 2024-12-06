# has-some
Implement the opposite of `is_empty` in order to allow better semantics for iterator filters and
other situations where `!T.is_empty()` is counterintuitive.

While using `is_empty` in an iterator `filter` method is relative verbose anyway, when that
iterator has a reference as an item type, it still stands that the semantics of "not is_empty"
are annoying (well, to me) when "has some" is clearer.

That's all this does - it provides the opposite of `is_empty` when what you really want to know
is "does it contain stuff".

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
