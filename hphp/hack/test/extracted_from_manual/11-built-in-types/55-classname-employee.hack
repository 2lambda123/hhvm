// @generated by hh_manual from manual/hack/11-built-in-types/55-classname.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
<<__ConsistentConstruct>>
class Employee {
  // ...
}

function f(classname<Employee> $clsname): void {
  $w = new $clsname();  // create an object whose type is passed in
}
