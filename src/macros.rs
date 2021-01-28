#[macro_export]
macro_rules! doc_comment {
    ($x:expr) => {
        #[doc = $x]
        extern {}
    };
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}
