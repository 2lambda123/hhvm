<?hh
/**
 * Copyright (c) 2014, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *
 */

function f(): void {
  switch (1) {
    case 1:
      switch (2) {
        case 1:
          return;
        case 2:
          break;
        default:
          throw new Exception('bye');
      }
    case 2:
      break;
  }
}
