#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[macro_use]
extern crate lazy_static;

macro_rules! define_error_codes {
    ($prefix:literal,
        $($name:ident => ($suffix:literal, $description:literal, $workaround:literal)),+
    ) => {
        use crate::ErrorCode;
        $(pub const $name: ErrorCode = ErrorCode {
            code: concat!($prefix, $suffix),
            description: $description,
            workaround: $workaround,
        };)+
        lazy_static! {
           pub static ref ALL_ERROR_CODES: Vec<ErrorCode> = vec![$($name,)+];
        }
    };
}

pub const UNKNOWN: ErrorCode = ErrorCode {
    code: "KV:Unknown",
    description: "",
    workaround: "",
};

pub mod engine;

use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct ErrorCode {
    pub code: &'static str,
    pub description: &'static str,
    pub workaround: &'static str,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

pub trait ErrorCodeExt {
    fn error_code(&self) -> ErrorCode;
}