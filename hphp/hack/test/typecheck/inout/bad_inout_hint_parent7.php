<?hh

class C1 {
  public function foo(): (function(inout int): void) {
    return (inout $x) ==> {
    };
  }
}

class C2 extends C1 {
  public function foo(): (function(inout arraykey): void) {
    return (inout $x) ==> {
    };
  }
}
