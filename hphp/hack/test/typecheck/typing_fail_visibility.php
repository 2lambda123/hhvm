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

class A {
  private function f(): void;
}

class B extends A {
  public function g(): void {
    $x = new A();
    $x->f();
  }
}
