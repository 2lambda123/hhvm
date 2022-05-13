<?hh // partial

namespace {

interface IExceptionWithPureGetMessage {
  require extends Exception;

  public function getMessage()[]: string;
}

trait ExceptionWithPureGetMessageTrait implements IExceptionWithPureGetMessage {
  require extends Exception;

  public function getMessage()[]: string {
    return $this->message;
  }
}

class ExceptionWithPureGetMessage extends Exception {
  use ExceptionWithPureGetMessageTrait;
}

class Exception implements Throwable {
  use \__SystemLib\BaseException;

  private static $traceOpts = 0;

  final public static function getTraceOptions()[read_globals] : int {
    return (readonly self::$traceOpts) as int;
  }

  final public static function setTraceOptions($opts)[globals] {
    self::$traceOpts = (int)$opts;
  }

  // This doc comment block generated by idl/sysdoc.php
  /**
   * ( excerpt from http://php.net/manual/en/exception.construct.php )
   *
   * Constructs the Exception.
   *
   * @message    mixed   The Exception message to throw.
   * @code       mixed   The Exception code.
   * @previous   mixed   The previous exception used for the exception
   *                     chaining.
   */
  public function __construct($message = '', $code = 0,
                              ?Throwable $previous = null)[] {

    // Child classes may just override the protected property
    // without implementing a constructor or calling parent one.
    // In this case we should not override it from the param.

    if ($message !== '' || $this->message === '') {
      $this->message = $message;
    }

    if ($code !== 0 || $this->code === 0) {
      $this->code = $code;
    }

    $this->previous = $previous;
  }

  public static function toStringPure(
    Exception $e,
    ?(function(Throwable)[]:string) $fallback = null,
  )[]: string {
    return self::toStringFromGetMessage(
      $e,
      $throwable ==> {
        if ($throwable is IExceptionWithPureGetMessage) {
          return $throwable->getMessage();
        } else if ($fallback is nonnull) {
          return $fallback($throwable);
        } else {
          return 'Message ommitted because '.get_class($throwable).
            ' does not implement '.IExceptionWithPureGetMessage::class;
        }
      },
    );
  }
}

class DivisionByZeroException extends Exception {
  use ExceptionWithPureGetMessageTrait;
}

}
