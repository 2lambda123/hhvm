/**
 * Autogenerated by Thrift for thrift/compiler/test/fixtures/py3/src/module.thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated @nocommit
 */

#include "thrift/compiler/test/fixtures/py3/gen-py3cpp/module_data.h"

#include <thrift/lib/cpp2/gen/module_data_cpp.h>

FOLLY_CLANG_DISABLE_WARNING("-Wunused-macros")

#if defined(__GNUC__) && defined(__linux__) && !FOLLY_MOBILE
// These attributes are applied to the static data members to ensure that they
// are not stripped from the compiled binary, in order to keep them available
// for use by debuggers at runtime.
//
// The "used" attribute is required to ensure the compiler always emits unused
// data.
//
// The "section" attribute is required to stop the linker from stripping used
// data. It works by forcing all of the data members (both used and unused ones)
// into the same section. As the linker strips data on a per-section basis, it
// is then unable to remove unused data without also removing used data.
// This has a similar effect to the "retain" attribute, but works with older
// toolchains.
#define THRIFT_DATA_MEMBER [[gnu::used]] [[gnu::section(".rodata.thrift.data")]]
#else
#define THRIFT_DATA_MEMBER
#endif

namespace apache {
namespace thrift {

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::SimpleException>::fields_names = {{
  "err_code",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 1> TStructDataStorage<::py3::simple::SimpleException>::fields_ids = {{
  1,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 1> TStructDataStorage<::py3::simple::SimpleException>::fields_types = {{
  TType::T_I16,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::SimpleException>::storage_names = {{
  "__fbthrift_field_err_code",
}};
THRIFT_DATA_MEMBER const std::array<int, 1> TStructDataStorage<::py3::simple::SimpleException>::isset_indexes = {{
  0,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::OptionalRefStruct>::fields_names = {{
  "optional_blob",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 1> TStructDataStorage<::py3::simple::OptionalRefStruct>::fields_ids = {{
  1,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 1> TStructDataStorage<::py3::simple::OptionalRefStruct>::fields_types = {{
  TType::T_STRING,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::OptionalRefStruct>::storage_names = {{
  "__fbthrift_field_optional_blob",
}};
THRIFT_DATA_MEMBER const std::array<int, 1> TStructDataStorage<::py3::simple::OptionalRefStruct>::isset_indexes = {{
  0,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 8> TStructDataStorage<::py3::simple::SimpleStruct>::fields_names = {{
  "is_on",
  "tiny_int",
  "small_int",
  "nice_sized_int",
  "big_int",
  "real",
  "smaller_real",
  "hidden_field",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 8> TStructDataStorage<::py3::simple::SimpleStruct>::fields_ids = {{
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 8> TStructDataStorage<::py3::simple::SimpleStruct>::fields_types = {{
  TType::T_BOOL,
  TType::T_BYTE,
  TType::T_I16,
  TType::T_I32,
  TType::T_I64,
  TType::T_DOUBLE,
  TType::T_FLOAT,
  TType::T_I16,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 8> TStructDataStorage<::py3::simple::SimpleStruct>::storage_names = {{
  "__fbthrift_field_is_on",
  "__fbthrift_field_tiny_int",
  "__fbthrift_field_small_int",
  "__fbthrift_field_nice_sized_int",
  "__fbthrift_field_big_int",
  "__fbthrift_field_real",
  "__fbthrift_field_smaller_real",
  "__fbthrift_field_hidden_field",
}};
THRIFT_DATA_MEMBER const std::array<int, 8> TStructDataStorage<::py3::simple::SimpleStruct>::isset_indexes = {{
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::GeneratedStruct>::fields_names = {{
  "the",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 1> TStructDataStorage<::py3::simple::GeneratedStruct>::fields_ids = {{
  1,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 1> TStructDataStorage<::py3::simple::GeneratedStruct>::fields_types = {{
  TType::T_I16,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::GeneratedStruct>::storage_names = {{
  "__fbthrift_field_the",
}};
THRIFT_DATA_MEMBER const std::array<int, 1> TStructDataStorage<::py3::simple::GeneratedStruct>::isset_indexes = {{
  0,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::detail::AdaptedUnion>::fields_names = {{
  "best",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 1> TStructDataStorage<::py3::simple::detail::AdaptedUnion>::fields_ids = {{
  1,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 1> TStructDataStorage<::py3::simple::detail::AdaptedUnion>::fields_types = {{
  TType::T_I16,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::detail::AdaptedUnion>::storage_names = {{
  "best",
}};
THRIFT_DATA_MEMBER const std::array<int, 1> TStructDataStorage<::py3::simple::detail::AdaptedUnion>::isset_indexes = {{
  0,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::HiddenException>::fields_names = {{
  "test",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 1> TStructDataStorage<::py3::simple::HiddenException>::fields_ids = {{
  1,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 1> TStructDataStorage<::py3::simple::HiddenException>::fields_types = {{
  TType::T_I16,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::HiddenException>::storage_names = {{
  "__fbthrift_field_test",
}};
THRIFT_DATA_MEMBER const std::array<int, 1> TStructDataStorage<::py3::simple::HiddenException>::isset_indexes = {{
  0,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 9> TStructDataStorage<::py3::simple::ComplexStruct>::fields_names = {{
  "structOne",
  "structTwo",
  "an_integer",
  "name",
  "an_enum",
  "some_bytes",
  "from",
  "cdef",
  "bytes_with_cpp_type",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 9> TStructDataStorage<::py3::simple::ComplexStruct>::fields_ids = {{
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 9> TStructDataStorage<::py3::simple::ComplexStruct>::fields_types = {{
  TType::T_STRUCT,
  TType::T_STRUCT,
  TType::T_I32,
  TType::T_STRING,
  TType::T_I32,
  TType::T_STRING,
  TType::T_STRING,
  TType::T_STRING,
  TType::T_STRING,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 9> TStructDataStorage<::py3::simple::ComplexStruct>::storage_names = {{
  "__fbthrift_field_structOne",
  "__fbthrift_field_structTwo",
  "__fbthrift_field_an_integer",
  "__fbthrift_field_name",
  "__fbthrift_field_an_enum",
  "__fbthrift_field_some_bytes",
  "__fbthrift_field_from",
  "__fbthrift_field_cdef",
  "__fbthrift_field_bytes_with_cpp_type",
}};
THRIFT_DATA_MEMBER const std::array<int, 9> TStructDataStorage<::py3::simple::ComplexStruct>::isset_indexes = {{
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::BinaryUnion>::fields_names = {{
  "iobuf_val",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 1> TStructDataStorage<::py3::simple::BinaryUnion>::fields_ids = {{
  1,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 1> TStructDataStorage<::py3::simple::BinaryUnion>::fields_types = {{
  TType::T_STRING,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::BinaryUnion>::storage_names = {{
  "iobuf_val",
}};
THRIFT_DATA_MEMBER const std::array<int, 1> TStructDataStorage<::py3::simple::BinaryUnion>::isset_indexes = {{
  0,
}};

THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::BinaryUnionStruct>::fields_names = {{
  "u",
}};
THRIFT_DATA_MEMBER const std::array<int16_t, 1> TStructDataStorage<::py3::simple::BinaryUnionStruct>::fields_ids = {{
  1,
}};
THRIFT_DATA_MEMBER const std::array<protocol::TType, 1> TStructDataStorage<::py3::simple::BinaryUnionStruct>::fields_types = {{
  TType::T_STRUCT,
}};
THRIFT_DATA_MEMBER const std::array<folly::StringPiece, 1> TStructDataStorage<::py3::simple::BinaryUnionStruct>::storage_names = {{
  "__fbthrift_field_u",
}};
THRIFT_DATA_MEMBER const std::array<int, 1> TStructDataStorage<::py3::simple::BinaryUnionStruct>::isset_indexes = {{
  0,
}};

} // namespace thrift
} // namespace apache
