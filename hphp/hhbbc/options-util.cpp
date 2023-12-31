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
#include "hphp/hhbbc/options-util.h"

#include "hphp/hhbbc/options.h"
#include "hphp/hhbbc/representation.h"
#include "hphp/hhbbc/unit-util.h"

namespace HPHP::HHBBC {

//////////////////////////////////////////////////////////////////////

bool method_map_contains(const MethodMap& mmap,
                         const php::Class* cls,
                         const php::Func* func) {
  std::string const clsname = cls ? cls->name->data() : "";
  auto it = mmap.find(clsname);
  if (it == end(mmap)) return false;
  return it->second.count(func == nullptr ? "" :
                          (func->name->empty() ?
                           func->unit : func->name)->data());
}

bool is_trace_function(const php::Class* cls,
                       const php::Func* func) {
  return method_map_contains(options.TraceFunctions, cls, func);
}

int trace_bump_for(const php::Class* cls,
                   const php::Func* func) {
  auto const unit = func ? func->unit : cls->unit;
  return is_trace_function(cls, func) ? kTraceFuncBump :
    (is_systemlib_part(unit) ? kSystemLibBump : 0);
}

//////////////////////////////////////////////////////////////////////

}
