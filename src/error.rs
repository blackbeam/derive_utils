use std::{error, fmt, result};

use proc_macro2::TokenStream;
use syn;

pub(crate) type StdResult<T, E> = result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;

pub fn compile_err(msg: &str) -> TokenStream {
    quote!(compile_error!(#msg);)
}

#[derive(Debug)]
pub enum Error {
    /// `syn::Error`.
    Syn(syn::Error),
    /// other error.
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Syn(e) => write!(f, "{}", e),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Syn(e) => e.description(),
            Error::Other(s) => s,
        }
    }

    #[cfg(not(stable_1_30))]
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::Syn(e) => Some(e),
            Error::Other(_) => None,
        }
    }

    #[cfg(stable_1_30)]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Syn(e) => Some(e),
            Error::Other(_) => None,
        }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Error::Other(s.into())
    }
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Error::Syn(e)
    }
}
