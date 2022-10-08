/**
 * Autogenerated by Thrift for thrift/compiler/test/fixtures/maps-with-incomplete-types/src/module.thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated @nocommit
 */
#pragma once

#include "thrift/compiler/test/fixtures/maps-with-incomplete-types/gen-cpp2/module_types.h"

#include <thrift/lib/cpp2/gen/module_types_tcc.h>


namespace apache {
namespace thrift {
namespace detail {

template <>
struct TccStructTraits<::apache::thrift::test::A> {
  static void translateFieldName(
      folly::StringPiece _fname,
      int16_t& fid,
      apache::thrift::protocol::TType& _ftype) noexcept;
};
template <>
struct TccStructTraits<::apache::thrift::test::B> {
  static void translateFieldName(
      folly::StringPiece _fname,
      int16_t& fid,
      apache::thrift::protocol::TType& _ftype) noexcept;
};

} // namespace detail
} // namespace thrift
} // namespace apache

namespace apache { namespace thrift { namespace test {

template <class Protocol_>
void A::readNoXfer(Protocol_* iprot) {
  __fbthrift_clear_terse_fields();

  apache::thrift::detail::ProtocolReaderStructReadState<Protocol_> _readState;

  _readState.readStructBegin(iprot);

  using apache::thrift::TProtocolException;


  if (UNLIKELY(!_readState.advanceToNextField(
          iprot,
          0,
          1,
          apache::thrift::protocol::T_MAP))) {
    goto _loop;
  }
_readField_some_map:
  {
    _readState.beforeSubobject(iprot);
    this->__fbthrift_field_some_map = std::unordered_map<::std::int32_t, ::apache::thrift::test::B>();
    ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::map<::apache::thrift::type_class::integral, ::apache::thrift::type_class::structure>, std::unordered_map<::std::int32_t, ::apache::thrift::test::B>>::readWithContext(*iprot, this->__fbthrift_field_some_map, _readState);
    _readState.afterSubobject(iprot);
    
  }
 this->__isset.set(0, true);

  if (UNLIKELY(!_readState.advanceToNextField(
          iprot,
          1,
          0,
          apache::thrift::protocol::T_STOP))) {
    goto _loop;
  }

_end:
  _readState.readStructEnd(iprot);

  return;

_loop:
  _readState.afterAdvanceFailure(iprot);
  if (_readState.atStop()) {
    goto _end;
  }
  if (iprot->kUsesFieldNames()) {
    _readState.template fillFieldTraitsFromName<apache::thrift::detail::TccStructTraits<A>>();
  }

  switch (_readState.fieldId) {
    case 1:
    {
      if (LIKELY(_readState.isCompatibleWithType(iprot, apache::thrift::protocol::T_MAP))) {
        goto _readField_some_map;
      } else {
        goto _skip;
      }
    }
    default:
    {
_skip:
      _readState.skip(iprot);
      _readState.readFieldEnd(iprot);
      _readState.readFieldBeginNoInline(iprot);
      goto _loop;
    }
  }
}

template <class Protocol_>
uint32_t A::serializedSize(Protocol_ const* prot_) const {
  uint32_t xfer = 0;
  xfer += prot_->serializedStructSize("A");
  if (this->__isset.get(0)) {
    xfer += prot_->serializedFieldSize("some_map", apache::thrift::protocol::T_MAP, 1);
    xfer += ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::map<::apache::thrift::type_class::integral, ::apache::thrift::type_class::structure>, std::unordered_map<::std::int32_t, ::apache::thrift::test::B>>::serializedSize<false>(*prot_, this->__fbthrift_field_some_map);
  }
  xfer += prot_->serializedSizeStop();
  return xfer;
}

template <class Protocol_>
uint32_t A::serializedSizeZC(Protocol_ const* prot_) const {
  uint32_t xfer = 0;
  xfer += prot_->serializedStructSize("A");
  if (this->__isset.get(0)) {
    xfer += prot_->serializedFieldSize("some_map", apache::thrift::protocol::T_MAP, 1);
    xfer += ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::map<::apache::thrift::type_class::integral, ::apache::thrift::type_class::structure>, std::unordered_map<::std::int32_t, ::apache::thrift::test::B>>::serializedSize<false>(*prot_, this->__fbthrift_field_some_map);
  }
  xfer += prot_->serializedSizeStop();
  return xfer;
}

template <class Protocol_>
uint32_t A::write(Protocol_* prot_) const {
  uint32_t xfer = 0;
  xfer += prot_->writeStructBegin("A");
  bool previousFieldHasValue = true;
  if (this->__isset.get(0)) {
    constexpr int16_t kPrevFieldId = 0;
    xfer += ::apache::thrift::detail::writeFieldBegin<apache::thrift::protocol::T_MAP, 1, kPrevFieldId>(*prot_, "some_map", previousFieldHasValue);
    previousFieldHasValue = true;
    xfer += ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::map<::apache::thrift::type_class::integral, ::apache::thrift::type_class::structure>, std::unordered_map<::std::int32_t, ::apache::thrift::test::B>>::write(*prot_, this->__fbthrift_field_some_map);
    xfer += prot_->writeFieldEnd();
  } else {
    previousFieldHasValue = false;
  }
  xfer += prot_->writeFieldStop();
  xfer += prot_->writeStructEnd();
  return xfer;
}

extern template void A::readNoXfer<>(apache::thrift::BinaryProtocolReader*);
extern template uint32_t A::write<>(apache::thrift::BinaryProtocolWriter*) const;
extern template uint32_t A::serializedSize<>(apache::thrift::BinaryProtocolWriter const*) const;
extern template uint32_t A::serializedSizeZC<>(apache::thrift::BinaryProtocolWriter const*) const;
extern template void A::readNoXfer<>(apache::thrift::CompactProtocolReader*);
extern template uint32_t A::write<>(apache::thrift::CompactProtocolWriter*) const;
extern template uint32_t A::serializedSize<>(apache::thrift::CompactProtocolWriter const*) const;
extern template uint32_t A::serializedSizeZC<>(apache::thrift::CompactProtocolWriter const*) const;


template <class Protocol_>
void B::readNoXfer(Protocol_* iprot) {
  __fbthrift_clear_terse_fields();

  apache::thrift::detail::ProtocolReaderStructReadState<Protocol_> _readState;

  _readState.readStructBegin(iprot);

  using apache::thrift::TProtocolException;


  if (UNLIKELY(!_readState.advanceToNextField(
          iprot,
          0,
          1,
          apache::thrift::protocol::T_I32))) {
    goto _loop;
  }
_readField_field:
  {
    ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::integral, ::std::int32_t>::readWithContext(*iprot, this->__fbthrift_field_field, _readState);
    
  }
 this->__isset.set(0, true);

  if (UNLIKELY(!_readState.advanceToNextField(
          iprot,
          1,
          0,
          apache::thrift::protocol::T_STOP))) {
    goto _loop;
  }

_end:
  _readState.readStructEnd(iprot);

  return;

_loop:
  _readState.afterAdvanceFailure(iprot);
  if (_readState.atStop()) {
    goto _end;
  }
  if (iprot->kUsesFieldNames()) {
    _readState.template fillFieldTraitsFromName<apache::thrift::detail::TccStructTraits<B>>();
  }

  switch (_readState.fieldId) {
    case 1:
    {
      if (LIKELY(_readState.isCompatibleWithType(iprot, apache::thrift::protocol::T_I32))) {
        goto _readField_field;
      } else {
        goto _skip;
      }
    }
    default:
    {
_skip:
      _readState.skip(iprot);
      _readState.readFieldEnd(iprot);
      _readState.readFieldBeginNoInline(iprot);
      goto _loop;
    }
  }
}

template <class Protocol_>
uint32_t B::serializedSize(Protocol_ const* prot_) const {
  uint32_t xfer = 0;
  xfer += prot_->serializedStructSize("B");
  if (this->__isset.get(0)) {
    xfer += prot_->serializedFieldSize("field", apache::thrift::protocol::T_I32, 1);
    xfer += ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::integral, ::std::int32_t>::serializedSize<false>(*prot_, this->__fbthrift_field_field);
  }
  xfer += prot_->serializedSizeStop();
  return xfer;
}

template <class Protocol_>
uint32_t B::serializedSizeZC(Protocol_ const* prot_) const {
  uint32_t xfer = 0;
  xfer += prot_->serializedStructSize("B");
  if (this->__isset.get(0)) {
    xfer += prot_->serializedFieldSize("field", apache::thrift::protocol::T_I32, 1);
    xfer += ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::integral, ::std::int32_t>::serializedSize<false>(*prot_, this->__fbthrift_field_field);
  }
  xfer += prot_->serializedSizeStop();
  return xfer;
}

template <class Protocol_>
uint32_t B::write(Protocol_* prot_) const {
  uint32_t xfer = 0;
  xfer += prot_->writeStructBegin("B");
  bool previousFieldHasValue = true;
  if (this->__isset.get(0)) {
    constexpr int16_t kPrevFieldId = 0;
    xfer += ::apache::thrift::detail::writeFieldBegin<apache::thrift::protocol::T_I32, 1, kPrevFieldId>(*prot_, "field", previousFieldHasValue);
    previousFieldHasValue = true;
    xfer += ::apache::thrift::detail::pm::protocol_methods<::apache::thrift::type_class::integral, ::std::int32_t>::write(*prot_, this->__fbthrift_field_field);
    xfer += prot_->writeFieldEnd();
  } else {
    previousFieldHasValue = false;
  }
  xfer += prot_->writeFieldStop();
  xfer += prot_->writeStructEnd();
  return xfer;
}

extern template void B::readNoXfer<>(apache::thrift::BinaryProtocolReader*);
extern template uint32_t B::write<>(apache::thrift::BinaryProtocolWriter*) const;
extern template uint32_t B::serializedSize<>(apache::thrift::BinaryProtocolWriter const*) const;
extern template uint32_t B::serializedSizeZC<>(apache::thrift::BinaryProtocolWriter const*) const;
extern template void B::readNoXfer<>(apache::thrift::CompactProtocolReader*);
extern template uint32_t B::write<>(apache::thrift::CompactProtocolWriter*) const;
extern template uint32_t B::serializedSize<>(apache::thrift::CompactProtocolWriter const*) const;
extern template uint32_t B::serializedSizeZC<>(apache::thrift::CompactProtocolWriter const*) const;


}}} // apache::thrift::test
