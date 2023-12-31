<?hh

async function genx(): Awaitable<int> { return 42; }
async function geny(): Awaitable<int> { return 43; }

async function foo(): Awaitable<int> {
  concurrent {
    $x = await genx();
    $y = await geny();
  }
  return $x + $y;
}
