#![doc(html_no_source)]

#[doc(hidden)]
#[macro_use]
mod macros;

doc_comment! {
    include_str!("README.md"),
    pub mod posts {
        doc_comment! {
            include_str!("posts/api-security.md"),
            pub mod api_security {}
        }

        doc_comment! {
            include_str!("posts/crate-features.md"),
            pub mod crate_features {}
        }

        doc_comment! {
            include_str!("posts/pin-projection.md"),
            pub mod pin_projection {}
        }
    }
}

doc_comment! {
    include_str!("redbox/README.md"),
    pub mod redbox {
        doc_comment! {
            include_str!("redbox/chapter_00.md"),
            pub mod chapter_00 {}
        }

        doc_comment! {
            include_str!("redbox/chapter_01.md"),
            pub mod chapter_01 {}
        }

        doc_comment! {
            include_str!("redbox/chapter_02.md"),
            pub mod chapter_02 {}
        }

        doc_comment! {
            include_str!("redbox/chapter_03.md"),
            pub mod chapter_03 {}
        }
    }
}
