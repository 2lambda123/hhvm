<?hh

namespace HH {

  namespace ImplicitContext {

    async function soft_run_with_async<Tout>(
      (function()[_]: Awaitable<Tout>) $f,
      string $key,
    )[zoned, ctx $f]: Awaitable<Tout>;

    function soft_run_with<Tout>(
      (function()[_]: Tout) $f,
      string $key,
    )[zoned, ctx $f]: Tout;

  } // namespace ImplicitContext

  abstract class ImplicitContext {
    abstract const type T as nonnull;

    protected static async function runWithAsync<Tout>(
      this::T $context,
      (function()[_]: Awaitable<Tout>) $f,
    )[zoned, ctx $f]: Awaitable<Tout>;

    protected static function runWith<Tout>(
      this::T $context,
      (function()[_]: Tout) $f,
    )[zoned, ctx $f]: Tout;

    protected static function get()[zoned]: ?this::T;
  }

  /**
   * Options for memoization to be used with dynamically enforced implicit
   * context
   */
  enum class MemoizeOption: string {
    /**
     * Incorporate the Implicit Context state into the memoization cache key.
     */
    string KeyedByIC = 'KeyedByIC';
    /**
     * Do not incorporate the Implicit Context state into the memoization cache
     * key.
     * Attempting to fetch the Implicit Context in this function or a recursive
     * callee will result in an exception.
     * Calling an "uncategorized" memoized function including one using
     * #SoftMakeICInaccessible will result in an exception.
     */
    string MakeICInaccessible = 'MakeICInaccessible';
    /**
     * Will throw if called with an Implicit Context value.
     * Do not incorporate the Implicit Context state into the memoization cache
     * key.
     * Behaviors that would result in an exception under #MakeICInaccessible
     * will log instead.
     */
    string SoftMakeICInaccessible = 'SoftMakeICInaccessible';
    /**
     * Default option for memoization attributes.
     * Will throw if called with an Implicit Context value.
     * Do not incorporate the Implicit Context state into the memoization cache
     * key.
     */
    string Uncategorized = 'Uncategorized';
  }

} // namespace HH
