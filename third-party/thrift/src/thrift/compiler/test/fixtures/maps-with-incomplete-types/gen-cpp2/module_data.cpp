/**
 * Autogenerated by Thrift for thrift/compiler/test/fixtures/maps-with-incomplete-types/src/module.thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated @nocommit
 */

#include "thrift/compiler/test/fixtures/maps-with-incomplete-types/gen-cpp2/module_data.h"

#include <thrift/lib/cpp2/gen/module_data_cpp.h>

namespace apache {
namespace thrift {

const std::array<folly::StringPiece, 1> TStructDataStorage<::apache::thrift::test::A>::fields_names = {{
  "some_map",
}};
const std::array<int16_t, 1> TStructDataStorage<::apache::thrift::test::A>::fields_ids = {{
  1,
}};
const std::array<protocol::TType, 1> TStructDataStorage<::apache::thrift::test::A>::fields_types = {{
  TType::T_MAP,
}};
const std::array<folly::StringPiece, 1> TStructDataStorage<::apache::thrift::test::A>::storage_names = {{
  "__fbthrift_field_some_map",
}};
const std::array<int, 1> TStructDataStorage<::apache::thrift::test::A>::isset_indexes = {{
  0,
}};

const std::array<folly::StringPiece, 1> TStructDataStorage<::apache::thrift::test::B>::fields_names = {{
  "field",
}};
const std::array<int16_t, 1> TStructDataStorage<::apache::thrift::test::B>::fields_ids = {{
  1,
}};
const std::array<protocol::TType, 1> TStructDataStorage<::apache::thrift::test::B>::fields_types = {{
  TType::T_I32,
}};
const std::array<folly::StringPiece, 1> TStructDataStorage<::apache::thrift::test::B>::storage_names = {{
  "__fbthrift_field_field",
}};
const std::array<int, 1> TStructDataStorage<::apache::thrift::test::B>::isset_indexes = {{
  0,
}};

} // namespace thrift
} // namespace apache
