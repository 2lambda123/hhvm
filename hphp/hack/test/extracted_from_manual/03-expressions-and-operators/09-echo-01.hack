// @generated by hh_manual from manual/hack/03-expressions-and-operators/09-echo.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  $v1 = true;
  $v2 = 123.45;
  echo '>>'.$v1.'|'.$v2."<<\n"; // outputs ">>1|123.45<<"
  
  $v3 = "abc{$v2}xyz";
  echo "$v3\n";
}
