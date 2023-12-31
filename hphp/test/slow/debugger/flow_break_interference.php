<?hh


function foo($a) :mixed{
  return $a + 2;
}

function genFoo($a) :AsyncGenerator<mixed,mixed,void>{
  $a = foo($a);
  $z = yield $a+5;
  // Step out from this line will cause us to handle an iternext, with
  // a fall-thru destination of the first opcode of the line after the
  // foreach in main(). If flow control interferes with breakpoints,
  // then we'll execute that opcode (load $a) before stopping.
  yield HH\Lib\Legacy_FIXME\cast_for_arithmetic($z)+1;
  error_log('Finished in genFoo');
}

function main() :mixed{

  $a = 42;
  foreach (genFoo(1) as $x) {
    var_dump($x);
  }
  // If we're really stopped at the beginning of this line, then
  // changing $a will show in $z.
  $z = $a;
  var_dump($z);
}


<<__EntryPoint>>
function main_entry(): void {

  main();
}
