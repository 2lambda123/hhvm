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


namespace HPHP {
///////////////////////////////////////////////////////////////////////////////

/**
 * Returns whether or not the current request has a GET parameter
 * "enable_code_coverage" and its value is "true"
 */
bool isEnableCodeCoverageReqParamTrue();

/**
 * Returns whether or not the current request has a GET parameter
 * "enable_per_file_coverage" and its value is "true"
 */
bool isEnablePerFileCoverageReqParamTrue();

///////////////////////////////////////////////////////////////////////////////
}
