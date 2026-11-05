//! Loopback OAuth callback validation and cryptographic PKCE utilities.
mod entropy;
mod parse;
mod read;
mod receive;
mod reply;
mod request;
pub(crate) use entropy::{challenge, random_secret};
pub(crate) use receive::receive;

enum Callback {
    Code(String),
    Denied,
}

#[cfg(test)]
mod pkce_tests;
#[cfg(test)]
mod tests;
