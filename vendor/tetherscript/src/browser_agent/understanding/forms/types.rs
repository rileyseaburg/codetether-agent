//! Form classification types.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormPurpose {
    Login,
    Search,
    Registration,
    Checkout,
    Newsletter,
    Contact,
    Other,
}
