// @generated by hh_manual from manual/hack/04-statements/08-switch.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  $v = 10;
  switch ($v) {
    case 10:
      // ...
      // FALLTHROUGH
    case 30:
      // ...         // Handle 10 or 30
      break;
    default:
      // ...
      break;
  }
}
