// @generated by hh_manual from manual/hack/11-built-in-types/71-dynamic.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
function f(dynamic $x) : void {
  $n = $x + 5;            // $n is a num
  $s = $x . "hello!";     // $s is a string
  $y = $x->anyMethod();   // $y is dynamic
}
