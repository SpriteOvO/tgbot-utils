use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum CmdArgError {
    #[error("unrecognized or ill-formed argument: {0}")]
    UnrecognizedOrIllFormed(String),

    #[error("+/-option with =value is not supported yet")]
    UnsupportedBoolKVOption,

    #[error("{0}")]
    Download(#[from] teloxide::errors::DownloadError),
}

type Result<T> = std::result::Result<T, CmdArgError>;

pub trait Args: Default {
    fn help() -> &'static str;

    fn parse(input: impl AsRef<str>) -> Result<Self>;

    fn parse_inner(
        input: impl AsRef<str>,
        predicate: impl Fn(&mut Self, &str, Option<&ArgValue>) -> bool,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut args = Self::default();

        let parsed_args: Result<Vec<_>> =
            input.as_ref().split_whitespace().map(Arg::parse).collect();

        for arg in parsed_args?.into_iter() {
            if !predicate(&mut args, &arg.name, arg.value.as_ref()) {
                return Err(CmdArgError::UnrecognizedOrIllFormed(arg.to_string()));
            }
        }

        Ok(args)
    }
}

pub struct Arg {
    name: String,
    value: Option<ArgValue>,
}

impl Arg {
    fn parse(input: impl AsRef<str>) -> Result<Self> {
        let input = input.as_ref();

        let (sign, kv) = (
            {
                let mut iter = input.chars();
                iter.next()
                    .filter(|&ch| ch == '+' || ch == '-')
                    .zip(Some(iter.as_str()))
            },
            input.split_once('='),
        );

        let arg = match (sign, kv) {
            (None, Some((name, value))) => Self {
                name: name.into(),
                value: Some(ArgValue::KV(value.into())),
            },
            (Some((ch, name)), None) => Self {
                name: name.into(),
                value: Some(ArgValue::Bool(ch == '+')),
            },
            (Some(_), Some(_)) => {
                return Err(CmdArgError::UnsupportedBoolKVOption);
            }
            (None, None) => Self {
                name: input.into(),
                value: None,
            },
        };

        Ok(arg)
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(ArgValue::Bool(enable)) => {
                write!(f, "{}{}", *enable, self.name)
            }
            Some(ArgValue::KV(value)) => write!(f, "{}={}", self.name, value),
            None => write!(f, "{}", self.name),
        }
    }
}

pub enum ArgValue {
    Bool(bool), // `-opt`, `+opt`
    KV(String), // `arg=abc`
}

#[macro_export]
macro_rules! define_cmd_args {
    ( $help:literal $(#[$attrs:meta])* $vis:vis struct $name:ident { $($body:tt)* } ) => {
        $(#[$attrs])*
        $vis struct $name { $($body)* }

        impl $crate::cmd_arg::Args for $name {
            fn help() -> &'static str {
                $help
            }

            fn parse(input: impl AsRef<str>) -> std::result::Result<Self, $crate::cmd_arg::CmdArgError> {
                Self::parse_inner(input, |args, name, value| {
                    define_cmd_args!(@ARM, (name, value), args, $($body)*)
                })
            }
        }
    };
    ( @ARM, $input:expr, $result:expr,
      $(#[$attrs:meta])* $vis:vis $name:ident : bool, $($body:tt)*) => {
        if let (stringify!($name), None) = $input {
            $result.$name = true;
            return true;
        } else {
            define_cmd_args!(@ARM, $input, $result, $($body)*)
        }
    };
    ( @ARM, $input:expr, $result:expr,
      $(#[$attrs:meta])* $vis:vis $name:ident : Option<bool>, $($body:tt)*) => {
        if let (stringify!($name), Some($crate::cmd_arg::ArgValue::Bool(enable))) = $input {
            $result.$name = Some(*enable);
            return true;
        } else {
            define_cmd_args!(@ARM, $input, $result, $($body)*)
        }
    };
    ( @ARM, $input:expr, $result:expr,
      $(#[$attrs:meta])* $vis:vis $name:ident : Option<String>, $($body:tt)*) => {
        if let (stringify!($name), Some(ArgValue::KV(value))) = $input {
            $result.$name = Some(value.into());
            return true;
        } else {
            define_cmd_args!(@ARM, $input, $result, $($body)*)
        }
    };
    ( @ARM, $input:expr, $result:expr,) => {
      false
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    define_cmd_args! {
        "help text"

        #[derive(PartialEq, Eq, Debug, Default)]
        pub struct TestArgs {
            pub help: bool,
            opt_bool: Option<bool>,
            opt_string: Option<String>,
        }
    }

    #[test]
    fn validation() {
        assert_eq!(TestArgs::help(), "help text");
        assert_eq!(
            TestArgs::parse("").unwrap(),
            TestArgs {
                help: false,
                opt_bool: None,
                opt_string: None,
            }
        );

        assert_eq!(
            TestArgs::parse("help").unwrap(),
            TestArgs {
                help: true,
                opt_bool: None,
                opt_string: None,
            }
        );
        assert!(TestArgs::parse("+help").is_err());
        assert!(TestArgs::parse("-help").is_err());
        assert!(TestArgs::parse("help=abc").is_err());

        assert_eq!(
            TestArgs::parse("+opt_bool").unwrap(),
            TestArgs {
                help: false,
                opt_bool: Some(true),
                opt_string: None,
            }
        );
        assert_eq!(
            TestArgs::parse("-opt_bool").unwrap(),
            TestArgs {
                help: false,
                opt_bool: Some(false),
                opt_string: None,
            }
        );
        assert!(TestArgs::parse("opt_bool").is_err());
        assert!(TestArgs::parse("opt_bool=abc").is_err());

        assert_eq!(
            TestArgs::parse("opt_string=abc").unwrap(),
            TestArgs {
                help: false,
                opt_bool: None,
                opt_string: Some("abc".into()),
            }
        );
        assert!(TestArgs::parse("opt_string").is_err());
        assert!(TestArgs::parse("+opt_string").is_err());
        assert!(TestArgs::parse("-opt_string").is_err());
    }
}
