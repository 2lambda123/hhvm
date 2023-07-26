// @generated by Thrift for thrift/compiler/test/fixtures/rust-raw-identifiers/src/mod.thrift
// This file is probably not the place you want to edit!

//! Thrift service definitions for `mod`.


/// Service definitions for `Foo`.
pub mod foo {
    #[derive(Clone, Debug)]
    pub enum ReturnExn {
        #[doc(hidden)]
        Success(()),
        ApplicationException(::fbthrift::ApplicationException),
    }

    impl ::std::convert::From<crate::errors::foo::ReturnError> for ReturnExn {
        fn from(err: crate::errors::foo::ReturnError) -> Self {
            match err {
                crate::errors::foo::ReturnError::ApplicationException(aexn) => ReturnExn::ApplicationException(aexn),
                crate::errors::foo::ReturnError::ThriftError(err) => ReturnExn::ApplicationException(::fbthrift::ApplicationException {
                    message: err.to_string(),
                    type_: ::fbthrift::ApplicationExceptionErrorCode::InternalError,
                }),
            }
        }
    }

    impl ::std::convert::From<::fbthrift::ApplicationException> for ReturnExn {
        fn from(exn: ::fbthrift::ApplicationException) -> Self {
            Self::ApplicationException(exn)
        }
    }

    impl ::fbthrift::ExceptionInfo for ReturnExn {
        fn exn_name(&self) -> &'static str {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_name called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_name(),
            }
        }

        fn exn_value(&self) -> String {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_value called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_value(),
            }
        }

        fn exn_is_declared(&self) -> bool {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_is_declared called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_is_declared(),
            }
        }
    }

    impl ::fbthrift::ResultInfo for ReturnExn {
        fn result_type(&self) -> ::fbthrift::ResultType {
            match self {
                Self::Success(_) => ::fbthrift::ResultType::Return,
                Self::ApplicationException(_aexn) => ::fbthrift::ResultType::Exception,
            }
        }
    }

    impl ::fbthrift::GetTType for ReturnExn {
        const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
    }

    impl<P> ::fbthrift::Serialize<P> for ReturnExn
    where
        P: ::fbthrift::ProtocolWriter,
    {
        fn write(&self, p: &mut P) {
            if let Self::ApplicationException(aexn) = self {
                return aexn.write(p);
            }
            p.write_struct_begin("Return");
            match self {
                Self::Success(inner) => {
                    p.write_field_begin(
                        "Success",
                        ::fbthrift::TType::Void,
                        0i16,
                    );
                    inner.write(p);
                    p.write_field_end();
                }
                Self::ApplicationException(_aexn) => unreachable!(),
            }
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P> ::fbthrift::Deserialize<P> for ReturnExn
    where
        P: ::fbthrift::ProtocolReader,
    {
        fn read(p: &mut P) -> ::anyhow::Result<Self> {
            static RETURNS: &[::fbthrift::Field] = &[
                ::fbthrift::Field::new("Success", ::fbthrift::TType::Void, 0),
            ];
            let _ = p.read_struct_begin(|_| ())?;
            let mut once = false;
            let mut alt = Self::Success(());
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| (), RETURNS)?;
                match ((fty, fid as ::std::primitive::i32), once) {
                    ((::fbthrift::TType::Stop, _), _) => {
                        p.read_field_end()?;
                        break;
                    }
                    ((::fbthrift::TType::Void, 0i32), false) => {
                        once = true;
                        alt = Self::Success(::fbthrift::Deserialize::read(p)?);
                    }
                    ((ty, _id), false) => p.skip(ty)?,
                    ((badty, badid), true) => return ::std::result::Result::Err(::std::convert::From::from(
                        ::fbthrift::ApplicationException::new(
                            ::fbthrift::ApplicationExceptionErrorCode::ProtocolError,
                            format!(
                                "unwanted extra union {} field ty {:?} id {}",
                                "ReturnExn",
                                badty,
                                badid,
                            ),
                        )
                    )),
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            ::std::result::Result::Ok(alt)
        }
    }

    #[derive(Clone, Debug)]
    pub enum SuperExn {
        #[doc(hidden)]
        Success(()),
        ApplicationException(::fbthrift::ApplicationException),
    }

    impl ::std::convert::From<crate::errors::foo::SuperError> for SuperExn {
        fn from(err: crate::errors::foo::SuperError) -> Self {
            match err {
                crate::errors::foo::SuperError::ApplicationException(aexn) => SuperExn::ApplicationException(aexn),
                crate::errors::foo::SuperError::ThriftError(err) => SuperExn::ApplicationException(::fbthrift::ApplicationException {
                    message: err.to_string(),
                    type_: ::fbthrift::ApplicationExceptionErrorCode::InternalError,
                }),
            }
        }
    }

    impl ::std::convert::From<::fbthrift::ApplicationException> for SuperExn {
        fn from(exn: ::fbthrift::ApplicationException) -> Self {
            Self::ApplicationException(exn)
        }
    }

    impl ::fbthrift::ExceptionInfo for SuperExn {
        fn exn_name(&self) -> &'static str {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_name called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_name(),
            }
        }

        fn exn_value(&self) -> String {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_value called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_value(),
            }
        }

        fn exn_is_declared(&self) -> bool {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_is_declared called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_is_declared(),
            }
        }
    }

    impl ::fbthrift::ResultInfo for SuperExn {
        fn result_type(&self) -> ::fbthrift::ResultType {
            match self {
                Self::Success(_) => ::fbthrift::ResultType::Return,
                Self::ApplicationException(_aexn) => ::fbthrift::ResultType::Exception,
            }
        }
    }

    impl ::fbthrift::GetTType for SuperExn {
        const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
    }

    impl<P> ::fbthrift::Serialize<P> for SuperExn
    where
        P: ::fbthrift::ProtocolWriter,
    {
        fn write(&self, p: &mut P) {
            if let Self::ApplicationException(aexn) = self {
                return aexn.write(p);
            }
            p.write_struct_begin("Super");
            match self {
                Self::Success(inner) => {
                    p.write_field_begin(
                        "Success",
                        ::fbthrift::TType::Void,
                        0i16,
                    );
                    inner.write(p);
                    p.write_field_end();
                }
                Self::ApplicationException(_aexn) => unreachable!(),
            }
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P> ::fbthrift::Deserialize<P> for SuperExn
    where
        P: ::fbthrift::ProtocolReader,
    {
        fn read(p: &mut P) -> ::anyhow::Result<Self> {
            static RETURNS: &[::fbthrift::Field] = &[
                ::fbthrift::Field::new("Success", ::fbthrift::TType::Void, 0),
            ];
            let _ = p.read_struct_begin(|_| ())?;
            let mut once = false;
            let mut alt = Self::Success(());
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| (), RETURNS)?;
                match ((fty, fid as ::std::primitive::i32), once) {
                    ((::fbthrift::TType::Stop, _), _) => {
                        p.read_field_end()?;
                        break;
                    }
                    ((::fbthrift::TType::Void, 0i32), false) => {
                        once = true;
                        alt = Self::Success(::fbthrift::Deserialize::read(p)?);
                    }
                    ((ty, _id), false) => p.skip(ty)?,
                    ((badty, badid), true) => return ::std::result::Result::Err(::std::convert::From::from(
                        ::fbthrift::ApplicationException::new(
                            ::fbthrift::ApplicationExceptionErrorCode::ProtocolError,
                            format!(
                                "unwanted extra union {} field ty {:?} id {}",
                                "SuperExn",
                                badty,
                                badid,
                            ),
                        )
                    )),
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            ::std::result::Result::Ok(alt)
        }
    }
}
