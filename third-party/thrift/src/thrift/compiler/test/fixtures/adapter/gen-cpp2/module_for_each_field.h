/**
 * Autogenerated by Thrift for thrift/compiler/test/fixtures/adapter/src/module.thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated @nocommit
 */
#pragma once

#include "thrift/compiler/test/fixtures/adapter/gen-cpp2/module_metadata.h"
#include <thrift/lib/cpp2/visitation/for_each.h>

namespace apache {
namespace thrift {
namespace detail {

template <>
struct ForEachField<::facebook::thrift::test::MyAnnotation> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).signature_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::Foo> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).intField_ref()...);
    f(1, static_cast<T&&>(t).optionalIntField_ref()...);
    f(2, static_cast<T&&>(t).intFieldWithDefault_ref()...);
    f(3, static_cast<T&&>(t).setField_ref()...);
    f(4, static_cast<T&&>(t).optionalSetField_ref()...);
    f(5, static_cast<T&&>(t).mapField_ref()...);
    f(6, static_cast<T&&>(t).optionalMapField_ref()...);
    f(7, static_cast<T&&>(t).binaryField_ref()...);
    f(8, static_cast<T&&>(t).longField_ref()...);
    f(9, static_cast<T&&>(t).adaptedLongField_ref()...);
    f(10, static_cast<T&&>(t).doubleAdaptedField_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::Baz> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).intField_ref()...);
    f(1, static_cast<T&&>(t).setField_ref()...);
    f(2, static_cast<T&&>(t).mapField_ref()...);
    f(3, static_cast<T&&>(t).binaryField_ref()...);
    f(4, static_cast<T&&>(t).longField_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::Bar> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).structField_ref()...);
    f(1, static_cast<T&&>(t).optionalStructField_ref()...);
    f(2, static_cast<T&&>(t).structListField_ref()...);
    f(3, static_cast<T&&>(t).optionalStructListField_ref()...);
    f(4, static_cast<T&&>(t).unionField_ref()...);
    f(5, static_cast<T&&>(t).optionalUnionField_ref()...);
    f(6, static_cast<T&&>(t).adaptedStructField_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::detail::DirectlyAdapted> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).field_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::StructWithFieldAdapter> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).field_ref()...);
    f(1, static_cast<T&&>(t).shared_field_ref()...);
    f(2, static_cast<T&&>(t).opt_shared_field_ref()...);
    f(3, static_cast<T&&>(t).opt_boxed_field_ref()...);
    f(4, static_cast<T&&>(t).boxed_field_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::TerseAdaptedFields> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).int_field_ref()...);
    f(1, static_cast<T&&>(t).string_field_ref()...);
    f(2, static_cast<T&&>(t).set_field_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::B> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).a_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::A> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
  }
};

template <>
struct ForEachField<::facebook::thrift::test::Config> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).path_ref()...);
  }
};

template <>
struct ForEachField<::facebook::thrift::test::MyStruct> {
  template <typename F, typename... T>
  void operator()(FOLLY_MAYBE_UNUSED F&& f, FOLLY_MAYBE_UNUSED T&&... t) const {
    f(0, static_cast<T&&>(t).field_ref()...);
    f(1, static_cast<T&&>(t).set_string_ref()...);
  }
};
} // namespace detail
} // namespace thrift
} // namespace apache
