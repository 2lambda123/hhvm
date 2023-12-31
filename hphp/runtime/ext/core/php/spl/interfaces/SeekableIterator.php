<?hh

// This doc comment block generated by idl/sysdoc.php
/**
 * ( excerpt from http://php.net/manual/en/class.seekableiterator.php )
 *
 * The Seekable iterator.
 *
 */
interface SeekableIterator extends \HH\Iterator {
  // This doc comment block generated by idl/sysdoc.php
  /**
   * ( excerpt from http://php.net/manual/en/seekableiterator.seek.php )
   *
   * Seeks to a given position in the iterator.
   *
   * @position   mixed   The position to seek to.
   *
   * @return     mixed   No value is returned.
   */
  public function seek($position);
}
