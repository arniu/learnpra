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

doc_comment! {
    include_str!("redbox/README.md"),
    pub mod redbox {
        doc_comment! {
            include_str!("redbox/create-server.md"),
            pub mod chapter_01 {}
        }

        doc_comment! {
            include_str!("redbox/handle-message.md"),
            pub mod chapter_02 {}
        }

        doc_comment! {
            include_str!("redbox/support-lua.md"),
            pub mod chapter_03 {}
        }

        doc_comment! {
            include_str!("redbox/support-graphql.md"),
            pub mod chapter_04 {}
        }
    }
}
