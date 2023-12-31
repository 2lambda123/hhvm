// @generated by hh_manual from manual/hack/03-expressions-and-operators/91-coalesce.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  $arr = dict['black' => 10, 'white' => null];
  \print_r(vec[
    idx($arr, 'black', -100),  // 10
    idx($arr, 'white', -200),  // null
    idx($arr, 'green', -300),  // -300
    idx($arr, 'green'),        // null
  ]);
}
