<?hh

class LazySliceKeyedIterable implements \HH\KeyedIterable {
  use LazyKeyedIterable;

  private $iterable;
  private $start;
  private $len;

  public function __construct($iterable, $start, $len)[] {
    $this->iterable = $iterable;
    $this->start = $start;
    $this->len = $len;
  }
  public function getIterator()[] {
    return new LazySliceKeyedIterator($this->iterable->getIterator(),
                                      $this->start,
                                      $this->len);
  }
}
