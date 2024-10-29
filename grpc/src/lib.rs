#![allow(clippy::derive_partial_eq_without_eq)]

pub mod basic {
    include!("./proto-gen/basic.rs");
}

pub mod query {
    include!("./proto-gen/query.rs");
}
