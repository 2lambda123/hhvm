<?hh

module MLT_B;

<<__EntryPoint>>
function bar(): void {
  include 'module_level_traits_modules.inc';
  include 'module_level_traits_5.inc0';
  include 'module_level_traits_5.inc1';

  (new C())->getFoo();  // should work...
}
