<?hh

function throwFromDoWhileLoop(bool $cond): void {
  while ($cond) {
    throw new Exception("DoWhileBody");
  }
}
