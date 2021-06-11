<?hh

<<__EntryPoint>>
function main() {
  switch (30) {
    case 30.0:
      echo "case 30.0\n";
      break;
    case 30:
      echo "case 30\n";
      break;
  }

  $x = new X();
  $x->bar(new stdClass);
  $x->bar($x);
  $x->bar(new X());
}

class X {
  function bar($arg) {
    switch ($this) {
      case $arg:
        echo "arg\n";
        break;
      default:
        echo "def\n";
    }
    switch ($arg) {
      case $this:
        echo "this\n";
        break;
      default:
        echo "def\n";
    }
  }
}
