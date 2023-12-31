/*
   +----------------------------------------------------------------------+
   | HipHop for PHP                                                       |
   +----------------------------------------------------------------------+
   | Copyright (c) 2010-present Facebook, Inc. (http://www.facebook.com)  |
   +----------------------------------------------------------------------+
   | This source file is subject to version 3.01 of the PHP license,      |
   | that is bundled with this package in the file LICENSE, and is        |
   | available through the world-wide-web at the following url:           |
   | http://www.php.net/license/3_01.txt                                  |
   | If you did not receive a copy of the PHP license and are unable to   |
   | obtain it through the world-wide-web, please send a note to          |
   | license@php.net so we can mail you a copy immediately.               |
   +----------------------------------------------------------------------+
*/

#include "hphp/runtime/vm/jit/switch-profile.h"

#include "hphp/runtime/vm/jit/target-profile.h"

#include "hphp/util/assertions.h"

#include <folly/ScopeGuard.h>

#include <algorithm>
#include <cstdlib>
#include <vector>

namespace HPHP::jit {

///////////////////////////////////////////////////////////////////////////////

std::vector<SwitchCaseCount>
sortedSwitchProfile(TargetProfile<SwitchProfile>& profile, int32_t nCases) {
  // SwitchProfile is variable-sized so we have to manually allocate it and
  // pass the buffer to TargetProfile::data().
  auto const size = sizeof(SwitchProfile) + SwitchProfile::extraSize(nCases);
  auto& data = *static_cast<SwitchProfile*>(calloc(1, size));
  SCOPE_EXIT { free(&data); };
  profile.data(data, size);

  std::vector<SwitchCaseCount> values;
  for (int i = 0; i < nCases; ++i) {
    values.emplace_back(SwitchCaseCount { i, data.cases()[i] });
  }
  std::sort(values.begin(), values.end());
  return values;
}

folly::dynamic SwitchProfile::toDynamic(uint32_t size) const {
 folly::dynamic casesArray = folly::dynamic::array;
 auto const nCases = size / sizeof(uint32_t);
 for (int i = 0; i < nCases; ++i) casesArray.push_back(cases()[i]);

 return folly::dynamic::object("profileType", "SwitchProfile")
                              ("cases", casesArray);
}

///////////////////////////////////////////////////////////////////////////////

}
