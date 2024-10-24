mod bytes;
mod derive_more;
mod enum_dispatch;
mod struct_builder;
mod strum;
mod this_error;
mod tracing_study;

mod axum;
mod demo;
mod serde;
mod tokio;

mod sqlx;
mod reqwest;
mod jwt;
mod argon;


pub const KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIMdPmgtf+UHuvnQECOh4HwmyE+sVSMqyLIwd6NPmuPHm
-----END PRIVATE KEY-----
";


pub const PUB_CEM: &str = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAKopxxA9Gq1Is45ytxFWWB/y0E0sGCl/b7FYiHzVa+uM=
-----END PUBLIC KEY-----
";