<?hh

trait Foo {
  public abstract static function bar(): void;

  public static function test(): void {
    static::bar<>;
  }
}
