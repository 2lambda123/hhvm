// @generated by Thrift for thrift/compiler/test/fixtures/rust-raw-identifiers/src/mod.thrift
// This file is probably not the place you want to edit!

//! Thrift error definitions for `mod`.

/// Error definitions for `Foo`.
pub mod foo {

    pub type ReturnError = ::fbthrift::NonthrowingFunctionError;

    impl ::std::convert::From<crate::services::foo::ReturnExn> for
        ::std::result::Result<(), ReturnError>
    {
        fn from(e: crate::services::foo::ReturnExn) -> Self {
            match e {
                crate::services::foo::ReturnExn::Success(res) => {
                    ::std::result::Result::Ok(res)
                }
                crate::services::foo::ReturnExn::ApplicationException(aexn) =>
                    ::std::result::Result::Err(ReturnError::ApplicationException(aexn)),
            }
        }
    }

    pub type SuperError = ::fbthrift::NonthrowingFunctionError;

    impl ::std::convert::From<crate::services::foo::SuperExn> for
        ::std::result::Result<(), SuperError>
    {
        fn from(e: crate::services::foo::SuperExn) -> Self {
            match e {
                crate::services::foo::SuperExn::Success(res) => {
                    ::std::result::Result::Ok(res)
                }
                crate::services::foo::SuperExn::ApplicationException(aexn) =>
                    ::std::result::Result::Err(SuperError::ApplicationException(aexn)),
            }
        }
    }

}

