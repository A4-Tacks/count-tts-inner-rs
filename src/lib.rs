#![doc = include_str!("../README.md")]

use proc_macro::{Span, TokenStream};
use proc_macro_tool::{pfunc, SetSpan, TokenTreeExt, Unsuffixed};

fn count_tts_impl(stream: TokenStream, span: Span) -> TokenStream {
    stream.into_iter()
        .count()
        .unsuffixed()
        .set_spaned(span)
        .tt()
        .into()
}

/// Expands to the number of token trees in the macro arguments
///
/// # Examples
///
/// ```
/// use count_tts_inner::count_tts;
///
/// assert_eq!(count_tts!(), 0);
/// assert_eq!(count_tts!(a b c), 3);
/// assert_eq!(count_tts!(a (b c)), 2);
/// assert_eq!(count_tts!(a, b, c), 5);
/// ```
#[proc_macro]
pub fn count_tts(stream: TokenStream) -> TokenStream {
    count_tts_impl(stream, Span::call_site())
}

/// Expands to the number of token trees in the inner like macro arguments
///
/// Can be used in places where macro expansion is not possible
///
/// # Examples
///
/// ```
/// use count_tts_inner::count_tts_inner;
///
/// macro_rules! foo {
///     ($t:literal) => { $t };
/// }
///
/// count_tts_inner! {
///     assert_eq!(0, foo!(#count_tts()));
///     assert_eq!(3, foo!(#count_tts(a b c)));
///     assert_eq!(2, foo!(#count_tts(a (b c))));
///     assert_eq!(5, foo!(#count_tts(a, b, c)));
/// }
/// ```
///
/// # Fail Cases
///
/// ```compile_fail
/// use count_tts_inner::count_tts;
///
/// macro_rules! foo {
///     ($t:literal) => { $t };
/// }
///
/// // Expected literal, `count_tts` `!` `()` is not a literal
/// assert_eq!(0, foo!(count_tts!()));
/// assert_eq!(3, foo!(count_tts!(a b c)));
/// assert_eq!(2, foo!(count_tts!(a (b c))));
/// assert_eq!(5, foo!(count_tts!(a, b, c)));
/// ```
#[proc_macro]
pub fn count_tts_inner(stream: TokenStream) -> TokenStream {
    pfunc(stream, false, ["count_tts"], |_, param| {
        count_tts_impl(param.stream(), param.span())
    })
}
