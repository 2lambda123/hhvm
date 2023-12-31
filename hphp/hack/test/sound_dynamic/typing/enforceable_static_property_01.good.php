<?hh
// (c) Meta Platforms, Inc. and affiliates. Confidential and proprietary.

<<__SupportDynamicType>>
abstract class C {
  abstract public function foo() : Awaitable<~int>;

  private static ?int $x;

  public async function bar(): Awaitable<~void> {
    self::$x = await $this->foo();
  }
}
