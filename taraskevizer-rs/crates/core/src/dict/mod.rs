pub mod flat_wordlist;
pub mod json;
pub mod loader;
pub mod phonetic;
pub mod types;
pub mod wordlist;

/// Define a batched wordlist with zero runtime overhead.
///
/// ```ignore
/// crate::dict_batches! {
///     pub WORD_LIST:
///     batch [
///         (r"аахен", r"аахэн"),
///         (" абасід", " абасыд"),
///     ],
///     batch [
///         (r"абанен([тц])", r"абанэн$1"),
///     ],
///     sequential [
///         (r" ге([^ ])", r"ґе$1"),
///     ]
/// }
/// ```
///
/// Expands to `pub const WORD_LIST: &[&[(&str, &str)]]` — plain static
/// slices, no code run at startup. Batches run in order, each in a single
/// pass (one combined regex per batch); `sequential` is the last batch
/// and keeps per-entry ordered semantics.
/// Move an entry between `batch` and `sequential` blocks to change its
/// execution model; order within and across blocks is preserved.
#[macro_export]
macro_rules! dict_batches {
    (
        $vis:vis $name:ident :
        $( batch [ $( ($p:expr, $r:expr) ),* $(,)? ] ),* $(,)?
        sequential [ $( ($sp:expr, $sr:expr) ),* $(,)? ]
        $(,)?
    ) => {
        $vis const $name: &[&[(&str, &str)]] = &[
            $( &[ $( ($p, $r)),* ], )*
            &[ $( ($sp, $sr)),* ],
        ];
    };
}

pub use loader::*;
pub use phonetic::PHONETIC;
pub use types::*;
pub use wordlist::WORD_LIST;
