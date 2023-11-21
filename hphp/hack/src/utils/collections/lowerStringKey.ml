(*
 * Copyright (c) 2018, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)
(* TODO(T170647909): In preparation to upgrading to ppx_yojson_conv.v0.16.X.
         Remove the suppress warning when the upgrade is done. *)
[@@@warning "-66"]

open Ppx_yojson_conv_lib.Yojson_conv.Primitives

type t = string [@@deriving yojson_of]

let compare (x : t) (y : t) =
  String.compare (String.lowercase_ascii x) (String.lowercase_ascii y)

let to_string t = String.lowercase_ascii t
