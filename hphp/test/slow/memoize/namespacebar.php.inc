<?hh


namespace Bar {
  trait TN {
    use \Foo\TN;
    public static $j = 30;
    <<__Memoize>>public function test($a = 0) {
      return $a + self::$j++;
    }
  }
}
