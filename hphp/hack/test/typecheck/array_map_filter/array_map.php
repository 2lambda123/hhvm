<?hh // partial

function expect_array_bool(varray<bool> $a): void {}
function test(
  (function(int, string): bool) $f,
  Container<int> $x,
  Container<string> $y,
): void {
  expect_array_bool(array_map($f, $x, $y));
}
