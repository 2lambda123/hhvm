// @generated by Thrift for thrift/compiler/test/fixtures/doctext/src/module.thrift
// This file is probably not the place you want to edit!

//! Thrift error definitions for `module`.

/// Error definitions for `C`.
pub mod c {

    pub trait AsBang {
        fn as_bang(&self) -> Option<&crate::types::Bang>;
    }

    impl AsBang for ::anyhow::Error {
        fn as_bang(&self) -> Option<&crate::types::Bang> {
            for cause in self.chain() {
                if let Some(ThingError::bang(e)) = cause.downcast_ref::<ThingError>() {
                    return Some(e);
                }
            }
            None
        }
    }

    pub type FError = ::fbthrift::NonthrowingFunctionError;

    impl ::std::convert::From<crate::services::c::FExn> for
        ::std::result::Result<(), FError>
    {
        fn from(e: crate::services::c::FExn) -> Self {
            match e {
                crate::services::c::FExn::Success(res) => {
                    ::std::result::Result::Ok(res)
                }
                crate::services::c::FExn::ApplicationException(aexn) =>
                    ::std::result::Result::Err(FError::ApplicationException(aexn)),
            }
        }
    }

    pub type NumbersError = ::fbthrift::NonthrowingFunctionError;

    impl ::std::convert::From<crate::services::c::NumbersExn> for
        ::std::result::Result<::futures::stream::BoxStream<'static, ::std::result::Result<crate::types::number, crate::errors::c::NumbersStreamError>>, NumbersError>
    {
        fn from(e: crate::services::c::NumbersExn) -> Self {
            match e {
                crate::services::c::NumbersExn::Success(res) => {
                    use ::futures::stream::StreamExt;
                    let stream = res;
                    ::std::result::Result::Ok(stream.map(|res| match res {
                        ::std::result::Result::Ok(item) => ::std::result::Result::Ok(item),
                        ::std::result::Result::Err(exn) => exn.into(),
                    }).boxed())
                }
                crate::services::c::NumbersExn::ApplicationException(aexn) =>
                    ::std::result::Result::Err(NumbersError::ApplicationException(aexn)),
            }
        }
    }

    impl ::std::convert::From<crate::services::c::NumbersResponseExn> for
        ::std::result::Result<(), NumbersError>
    {
        fn from(e: crate::services::c::NumbersResponseExn) -> Self {
            match e {
                crate::services::c::NumbersResponseExn::Success(res) =>
                    ::std::result::Result::Ok(res),
                crate::services::c::NumbersResponseExn::ApplicationException(aexn) =>
                    ::std::result::Result::Err(NumbersError::ApplicationException(aexn)),
            }
        }
    }

    pub type NumbersStreamError = ::fbthrift::NonthrowingFunctionError;

    impl ::std::convert::From<crate::services::c::NumbersStreamExn> for
        ::std::result::Result<crate::types::number, NumbersStreamError>
    {
        fn from(e: crate::services::c::NumbersStreamExn) -> Self {
            match e {
                crate::services::c::NumbersStreamExn::Success(res) =>
                    ::std::result::Result::Ok(res),
                crate::services::c::NumbersStreamExn::ApplicationException(aexn) =>
                    ::std::result::Result::Err(NumbersStreamError::ApplicationException(aexn)),
            }
        }
    }

    /// Errors for thing (client side).
    #[derive(Debug)]
    pub enum ThingError {
        bang(crate::types::Bang),
        ApplicationException(::fbthrift::types::ApplicationException),
        ThriftError(::anyhow::Error),
    }

    /// Human-readable string representation of the Thrift client error.
    ///
    /// By default, this will not print the full cause chain. If you would like to print the underlying error
    /// cause, either use `format!("{:?}", anyhow::Error::from(client_err))` or print this using the
    /// alternate formatter `{:#}` instead of just `{}`.
    impl ::std::fmt::Display for ThingError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
            match self {
                Self::bang(inner) => {
                    if f.alternate() {
                        write!(f, "C::thing failed with variant `bang`: {:#}", inner)?;
                    } else {
                        write!(f, "C::thing failed with bang(Bang)")?;
                    }
                }
                Self::ApplicationException(inner) => {
                    write!(f, "C::thing failed with ApplicationException")?;

                    if f.alternate() {
                      write!(f, ": {:#}", inner)?;
                    }
                }
                Self::ThriftError(inner) => {
                    write!(f, "C::thing failed with ThriftError")?;

                    if f.alternate() {
                      write!(f, ": {:#}", inner)?;
                    }
                }
            }

            Ok(())
        }
    }

    impl ::std::error::Error for ThingError {
        fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
            match self {
                Self::bang(ref inner) => {
                    Some(inner)
                }
                Self::ApplicationException(ref inner) => {
                    Some(inner)
                }
                Self::ThriftError(ref inner) => {
                    Some(inner.as_ref())
                }
            }
        }
    }

    impl ::std::convert::From<crate::types::Bang> for ThingError {
        fn from(e: crate::types::Bang) -> Self {
            Self::bang(e)
        }
    }

    impl AsBang for ThingError {
        fn as_bang(&self) -> Option<&crate::types::Bang> {
            match self {
                Self::bang(inner) => Some(inner),
                _ => None,
            }
        }
    }

    impl ::std::convert::From<::anyhow::Error> for ThingError {
        fn from(err: ::anyhow::Error) -> Self {
            Self::ThriftError(err)
        }
    }

    impl ::std::convert::From<::fbthrift::ApplicationException> for ThingError {
        fn from(ae: ::fbthrift::ApplicationException) -> Self {
            Self::ApplicationException(ae)
        }
    }
    impl ::std::convert::From<crate::services::c::ThingExn> for
        ::std::result::Result<::std::string::String, ThingError>
    {
        fn from(e: crate::services::c::ThingExn) -> Self {
            match e {
                crate::services::c::ThingExn::Success(res) => {
                    ::std::result::Result::Ok(res)
                }
                crate::services::c::ThingExn::ApplicationException(aexn) =>
                    ::std::result::Result::Err(ThingError::ApplicationException(aexn)),
                crate::services::c::ThingExn::bang(exn) =>
                    ::std::result::Result::Err(ThingError::bang(exn)),
            }
        }
    }

}

