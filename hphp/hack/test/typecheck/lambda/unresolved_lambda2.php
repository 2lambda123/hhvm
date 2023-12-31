<?hh

/**
 * Create nested Tunion that can occur as a result of anonymous
 * function call, and check that subsequent unification of those types works
 * correctly.
 */
function expect_arraykey(arraykey $k): void {}
function expect_float(float $f): void {}
function test(bool $b1, bool $b2): void {

  $make_int = function() {
    return 4;
  };

  $make_string = function() {
    return 'aaa';
  };

  $make_float = function() {
    return 3.0;
  };

  if ($b1) {
    $make_int_or_string = $make_int;
  } else {
    $make_int_or_string = $make_string;
  }

  if ($b2) {
    $f = $make_float;
    expect_float($f());
  } else {
    $f = $make_int_or_string;
    expect_arraykey($f());
  }
  // Todo: when we have explicit unions we can test float|string|int
}
