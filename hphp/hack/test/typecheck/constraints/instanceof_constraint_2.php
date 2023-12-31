<?hh
// Copyright 2004-present Facebook. All Rights Reserved.

interface I {}
class B<T as I> {}
class C<T as I> extends B<T> {}
function expects_C<T as I>(C<T> $c): void {}
function test_it<T as I>(B<T> $b): void {
  invariant($b is C<_>, 'broken');
  expects_C($b);
}
