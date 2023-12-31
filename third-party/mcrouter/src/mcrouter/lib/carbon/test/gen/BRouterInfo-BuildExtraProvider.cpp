/*
 *  Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 *  This source code is licensed under the MIT license found in the LICENSE
 *  file in the root directory of this source tree.
 *
 */

/*
 *  THIS FILE IS AUTOGENERATED. DO NOT MODIFY IT; ALL CHANGES WILL BE LOST IN
 *  VAIN.
 *
 *  @generated
 */
#include "BRouterInfo.h"

#include <mcrouter/routes/McExtraRouteHandleProvider.h>

using namespace facebook::memcache;
using namespace facebook::memcache::mcrouter;

namespace carbon {
namespace test {
namespace B {

/* static */
std::unique_ptr<facebook::memcache::mcrouter::ExtraRouteHandleProviderIf<BRouterInfo>>
BRouterInfo::buildExtraProvider() {
     return std::make_unique<McExtraRouteHandleProvider<BRouterInfo>>();
}

} // namespace B
} // namespace test
} // namespace carbon
