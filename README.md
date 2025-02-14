# i_splits

This crate add two string splitting methods:
- `split_i`: Split the string in half at the occurence `i` of a pattern.
- `split_once_last`: Split the string in half at the last occurence of a pattern.


This crate use no dependencies and no unsafe code. This comes at a compromise of having to return `String`s instead of `&str`s, as well as only being able to pass `&str`s as patterns. 
This is due to the fact that the `Pattern` trait isn't stable yet in std. 

## Example
```rust
use i_splits::ISplitExt as _;

// split_i

let v = "To show you the power of i_split, I cut that sentence in half!".split_i(", ", 1);
assert_eq!(v, Some(("To show you the power of i_split".to_string(), "I cut that sentence in half!".to_string())));

let v = "cookie|lolipop|muffin|pancake".split_i("|", 2);
assert_eq!(v, Some(("cookie|lolipop".to_string(), "muffin|pancake".to_string())));

let v = "No splits? That's a `None` for you".split_i("!", 2);
assert_eq!(v, None);

let v = "Don't go too far either!".split_i(" ", 10);
assert_eq!(v, None);

// split_once_last

let v = "To show you the power of i_split, I cut that sentence in half!".split_once_last(", ");
assert_eq!(v, Some(("To show you the power of i_split".to_string(), "I cut that sentence in half!".to_string())));

let v = "cookie|lolipop|muffin|pancake".split_once_last("|");
assert_eq!(v, Some(("cookie|lolipop|muffin".to_string(), "pancake".to_string())));
```