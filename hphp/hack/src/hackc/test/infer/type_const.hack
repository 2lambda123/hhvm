// RUN: %hackc compile-infer %s | FileCheck %s

// TEST-CHECK-BAL: type C$static
// CHECK: type C$static = .kind="class" .static {
// CHECK: }

abstract class C {
  <<__Enforceable>>
  abstract const type TMyShape;

  // TEST-CHECK-BAL: define C$static.check2
  // CHECK: define C$static.check2($this: *C$static, $a: *HackMixed) : *HackBool {
  // CHECK: #b0:
  // CHECK:   n0 = $builtins.hack_string("BaseType::Bool")
  // CHECK:   n1 = $builtins.hack_new_dict($builtins.hack_string("kind"), $builtins.hack_int(102), $builtins.hack_string("root_name"), $builtins.hack_string("self"), $builtins.hack_string("access_list"), $builtins.hhbc_new_vec($builtins.hack_string("TMyShape")))
  // CHECK:   n2: *HackMixed = load &$a
  // CHECK:   n3 = $builtins.hhbc_is_type_struct_c(n2, n1, $builtins.hack_int(1))
  // CHECK:   n4 = $root.todo(null, n0)
  // CHECK:   n5 = $builtins.hhbc_verify_type_pred(n3, n4)
  // CHECK:   ret n3
  // CHECK: }
  public static function check2(mixed $a): bool {
    return $a is self::TMyShape;
  }
}

// TEST-CHECK-BAL: type D$static
// CHECK: type D$static extends C$static = .kind="class" .static {
// CHECK: }

class D extends C {
  const type TMyShape = shape(
    ?'a' => ?string,
    ?'b' => ?int,
  );

  // TEST-CHECK-BAL: define D$static.check3
  // CHECK: define D$static.check3($this: *D$static, $shape: *HackMixed) : *void {
  // CHECK: #b0:
  // CHECK:   ret null
  // CHECK: }
  public static function check3(self::TMyShape $shape)[]: void {
  }
}


// TEST-CHECK-BAL: define $root.check1
// CHECK: define $root.check1($this: *void, $a: *HackMixed) : *HackBool {
// CHECK: #b0:
// CHECK:   n0 = $builtins.hack_string("BaseType::Bool")
// CHECK:   n1 = $builtins.hack_new_dict($builtins.hack_string("kind"), $builtins.hack_int(102), $builtins.hack_string("root_name"), $builtins.hack_string("D"), $builtins.hack_string("access_list"), $builtins.hhbc_new_vec($builtins.hack_string("TMyShape")))
// CHECK:   n2: *HackMixed = load &$a
// CHECK:   n3 = $builtins.hhbc_is_type_struct_c(n2, n1, $builtins.hack_int(1))
// CHECK:   n4 = $root.todo(null, n0)
// CHECK:   n5 = $builtins.hhbc_verify_type_pred(n3, n4)
// CHECK:   ret n3
// CHECK: }
function check1(mixed $a): bool {
  return $a is D::TMyShape;
}


// TEST-CHECK-BAL: define $root.main
// CHECK: define $root.main($this: *void) : *void {
// CHECK: #b0:
// CHECK:   n0 = $root.check1(null, $builtins.hack_int(5))
// CHECK:   n1: *D$static = load &D$static::static_singleton
// CHECK:   n2 = $builtins.lazy_initialize(n1)
// CHECK:   n3 = D$static.check2(n1, $builtins.hack_int(5))
// CHECK:   ret null
// CHECK: }
<<__EntryPoint>>
function main(): void {
  check1(5);
  D::check2(5);
}
