<?hh

function get(): int {
  return 1;
}

function f((int, int, int) $list_tuple = list(get(), 3, 4)): void{}
