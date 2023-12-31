<?hh
class C123<+T> {
  private T $x;
  public function __construct(T $x) {
    $this->x = $x;
  }
  public function get(): T {
    return $this->x;
  }
  public function foo<Tu super T>(Tu $x, bool $b): Tu {
    if ($b) {
      return $this->x;
    } else {
      return $x;
    }
  }
}
class FooBase123 {}
class Foo123 extends FooBase123 {}
class FooDerived123 extends Foo123 {}
class Woo {
  public function test1(
    C123<Foo123> $c,
    FooDerived123 $x,
    bool $b,
  ): FooDerived123 {
    return $c->foo($x, $b);
  }
}
