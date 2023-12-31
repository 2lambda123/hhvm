/*
   +----------------------------------------------------------------------+
   | HipHop for PHP                                                       |
   +----------------------------------------------------------------------+
   | Copyright (c) 2010-present Facebook, Inc. (http://www.facebook.com)  |
   | Copyright (c) 1997-2010 The PHP Group                                |
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

#include "hphp/runtime/ext/extension.h"

namespace HPHP {
///////////////////////////////////////////////////////////////////////////////

extern const int64_t k_PHP_URL_SCHEME;
extern const int64_t k_PHP_URL_HOST;
extern const int64_t k_PHP_URL_PORT;
extern const int64_t k_PHP_URL_USER;
extern const int64_t k_PHP_URL_PASS;
extern const int64_t k_PHP_URL_PATH;
extern const int64_t k_PHP_URL_QUERY;
extern const int64_t k_PHP_URL_FRAGMENT;
extern const int64_t k_PHP_QUERY_RFC1738;
extern const int64_t k_PHP_QUERY_RFC3986;

Variant HHVM_FUNCTION(http_build_query, const Variant& formdata,
                           const Variant& numeric_prefix /* = null_string */,
                           const String& arg_separator /* = null_string */,
                           int64_t enc_type = k_PHP_QUERY_RFC1738);
Variant HHVM_FUNCTION(parse_url, const String& url,
                                 int64_t component /* = -1 */);

///////////////////////////////////////////////////////////////////////////////
}
