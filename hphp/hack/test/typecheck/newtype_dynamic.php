//// def.php
<?hh
// Copyright 2004-present Facebook. All Rights Reserved.

newtype Foo as dynamic = dynamic;

//// use.php
<?hh
// Copyright 2004-present Facebook. All Rights Reserved.

function test(Foo $x): vec<dynamic> {
  return vec[$x];
}
