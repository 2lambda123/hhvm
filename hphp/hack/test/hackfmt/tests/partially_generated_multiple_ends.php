<?hh

// @partially-generated

// Only Foo::b and Foo::e are formatted--when we have multiple END tags in a
// row, we consider only the first.

class Foo {
  function a () : int { return 0; } // not formatted
  /* BEGIN MANUAL SECTION */
  function b () : int { return 0; }
  /* END MANUAL SECTION */
  function c () : int { return 0; } // not formatted
  /* END MANUAL SECTION */
  function d () : int { return 0; } // not formatted
  /* BEGIN MANUAL SECTION */
  function e () : int { return 0; }
  /* END MANUAL SECTION */
  function f () : int { return 0; } // not formatted
}
