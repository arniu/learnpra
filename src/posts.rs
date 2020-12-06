doc_comment! {
    include_str!("posts/api-security.md"),
    pub mod api_security {}
}

doc_comment! {
    include_str!("posts/no-default-features.md"),
    pub mod no_default_features {}
}

doc_comment! {
    include_str!("posts/redbox/README.md"),
    pub mod redbox {
        doc_comment! {
            include_str!("posts/redbox/create-server.md"),
            pub mod chapter_01 {}
        }

        doc_comment! {
            include_str!("posts/redbox/handle-message.md"),
            pub mod chapter_02 {}
        }

        doc_comment! {
            include_str!("posts/redbox/support-lua.md"),
            pub mod chapter_03 {}
        }

        doc_comment! {
            include_str!("posts/redbox/support-graphql.md"),
            pub mod chapter_04 {}
        }
    }
}
