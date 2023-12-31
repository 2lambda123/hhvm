<?hh /* -*- php -*- */
/**
 * Copyright (c) 2014, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 */

<<__PHPStdLib>>
function imap_8bit(string $str): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_alerts(): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_base64(string $text): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_binary(string $str): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_body(
  resource $imap_stream,
  int $msg_number,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_bodystruct(
  resource $imap_stream,
  int $msg_number,
  string $section,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_check(resource $imap_stream): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_clearflag_full(
  resource $imap_stream,
  string $sequence,
  string $flag,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_close(
  resource $imap_stream,
  int $flag = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_createmailbox(
  resource $imap_stream,
  string $mailbox,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_delete(
  resource $imap_stream,
  string $msg_number,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_deletemailbox(
  resource $imap_stream,
  string $mailbox,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_errors(): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_expunge(resource $imap_stream): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_fetch_overview(
  resource $imap_stream,
  string $sequence,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_fetchbody(
  resource $imap_stream,
  int $msg_number,
  string $section,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_fetchheader(
  resource $imap_stream,
  int $msg_number,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_fetchstructure(
  resource $imap_stream,
  int $msg_number,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_gc(
  resource $imap_stream,
  int $caches,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_header(
  resource $imap_stream,
  int $msg_number,
  int $fromlength = 0,
  int $subjectlength = 0,
  string $defaulthost = "",
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_headerinfo(
  resource $imap_stream,
  int $msg_number,
  int $fromlength = 0,
  int $subjectlength = 0,
  string $defaulthost = "",
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_last_error(): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_list(
  resource $imap_stream,
  string $ref,
  string $pattern,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_listmailbox(
  resource $imap_stream,
  string $ref,
  string $pattern,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_mail_copy(
  resource $imap_stream,
  string $msglist,
  string $mailbox,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_mail_move(
  resource $imap_stream,
  string $msglist,
  string $mailbox,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_mail(
  string $to,
  string $subject,
  string $message,
  string $additional_headers = "",
  string $cc = "",
  string $bcc = "",
  string $rpath = "",
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_mailboxmsginfo(
  resource $imap_stream,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_msgno(
  resource $imap_stream,
  int $uid,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_num_msg(resource $imap_stream): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_num_recent(resource $imap_stream): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_open(
  string $mailbox,
  string $username,
  string $password,
  int $options = 0,
  int $retries = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_ping(resource $imap_stream): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_qprint(string $str): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_renamemailbox(
  resource $imap_stream,
  string $old_mbox,
  string $new_mbox,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_reopen(
  resource $imap_stream,
  string $mailbox,
  int $options = 0,
  int $retries = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_search(
  resource $imap_stream,
  string $criteria,
  int $options = 0,
  string $charset = "",
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_setflag_full(
  resource $imap_stream,
  string $sequence,
  string $flag,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_status(
  resource $imap_stream,
  string $mailbox,
  int $options = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_subscribe(
  resource $imap_stream,
  string $mailbox,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_timeout(
  int $timeout_type,
  int $timeout = -1,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_uid(
  resource $imap_stream,
  int $msg_number,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_undelete(
  resource $imap_stream,
  string $msg_number,
  int $flags = 0,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_unsubscribe(
  resource $imap_stream,
  string $mailbox,
): HH\FIXME\MISSING_RETURN_TYPE;
<<__PHPStdLib>>
function imap_utf8(string $mime_encoded_text): HH\FIXME\MISSING_RETURN_TYPE;
