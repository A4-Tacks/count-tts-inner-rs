Expands to the number of token trees in the inner like macro arguments

Can be used in places where macro expansion is not possible

Similar [`count_tts`] crate

[`count_tts`]: https://crates.io/crates/count_tts

# Examples

Use [`count_tts`] failed case:

```rust,compile_fail
use count_tts::count_tts;

macro_rules! foo {
    ($t:literal) => { $t };
}

// Expected literal, `count_tts` `!` `()` is not a literal
assert_eq!(0, foo!(count_tts!()));
assert_eq!(3, foo!(count_tts!(a b c)));
assert_eq!(2, foo!(count_tts!(a (b c))));
assert_eq!(5, foo!(count_tts!(a, b, c)));
```

Use this crate case:

```rust
use count_tts_inner::count_tts_inner;

macro_rules! foo {
    ($t:literal) => { $t };
}

count_tts_inner! {
    assert_eq!(0, foo!(#count_tts()));
    assert_eq!(3, foo!(#count_tts(a b c)));
    assert_eq!(2, foo!(#count_tts(a (b c))));
    assert_eq!(5, foo!(#count_tts(a, b, c)));
}
```
