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
#pragma once

#if defined(__x86_64__)
extern "C" {
#include <xed-interface.h>
}
#endif // __x86_64__

#include <ostream>

namespace HPHP {

struct Disasm {
  struct Options {
    Options()
      : m_indentLevel(0)
      , m_printEncoding(false)
      , m_relativeOffset(false)
      , m_addresses(true)
      , m_forceAttSyntax(false)
    {}

    Options& indent(int i) {
      m_indentLevel = i;
      return *this;
    }

    Options& printEncoding(bool pe) {
      m_printEncoding = pe;
      return *this;
    }

    Options& relativeOffset(bool re) {
      m_relativeOffset = re;
      return *this;
    }

    Options& color(std::string c) {
      m_color = std::move(c);
      return *this;
    }

    Options& addresses(bool b) {
      m_addresses = b;
      return *this;
    }

    Options& forceAttSyntax(bool b) {
      m_forceAttSyntax = b;
      return *this;
    }

    int m_indentLevel;
    bool m_printEncoding;
    bool m_relativeOffset;
    bool m_addresses;
    bool m_forceAttSyntax;
    std::string m_color;
  };

  /* Create a Disasm object. indentLevel spaces will be put at the beginning of
   * each line of disassembly. If printEncoding is true, the raw hex bytes of
   * the instructions will also be in the output. */
  explicit Disasm(const Options& opts = Options());

  Disasm(const Disasm&) = delete;
  Disasm& operator=(const Disasm&) = delete;

  /* Disassemble instructions. start should be the first byte of the region to
   * disassemble and end should be the first byte past the region to
   * disassemble. */
  void disasm(std::ostream& out, uint8_t* start, uint8_t* end, uint64_t adjust);

  static void ExcludedAddressRange(void* low, size_t len);
 private:
#if defined(__x86_64__)
  xed_state_t m_xedState;
#endif // __x86_64__
  const Options m_opts;
};

} // namespace HPHP
