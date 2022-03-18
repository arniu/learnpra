#![doc(html_no_source)]

#[doc = include_str!("README.md")]
pub mod posts {
    #[doc = include_str!("posts/api_security.md")]
    pub mod api_security {}

    #[doc = include_str!("posts/crate_features.md")]
    pub mod crate_features {}

    #[doc = include_str!("posts/pin_projection.md")]
    pub mod pin_projection {}
}

#[doc = include_str!("redbox/README.md")]
pub mod redbox {
    #[doc = include_str!("redbox/chapter_00.md")]
    pub mod chapter_00 {}

    #[doc = include_str!("redbox/chapter_01.md")]
    pub mod chapter_01 {}

    #[doc = include_str!("redbox/chapter_02.md")]
    pub mod chapter_02 {}

    #[doc = include_str!("redbox/chapter_03.md")]
    pub mod chapter_03 {}
}
