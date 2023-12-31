<?hh

trait ATrait {
  public static $static_var = __CLASS__;
  public $var = __CLASS__;

  public static function get_class_name() :mixed{
    return __CLASS__;
  }

  public function get_class_name_obj() :mixed{
    return __CLASS__;
  }

  public static function get_class_name2() :mixed{
    return self::$static_var;
  }

  public function get_class_name_obj2() :mixed{
    return $this->var;
  }
}

trait Indirect {
    use ATrait;
}

class SomeClass {
   use ATrait;
}

class UsingIndirect {
    use Indirect;
}
<<__EntryPoint>> function main(): void {
$r = SomeClass::get_class_name();
var_dump($r);
$r = SomeClass::get_class_name2();
var_dump($r);

$o = new SomeClass();
$r = $o->get_class_name_obj();
var_dump($r);
$r = $o->get_class_name_obj2();
var_dump($r);

$r = UsingIndirect::get_class_name();
var_dump($r);
$r = UsingIndirect::get_class_name2();
var_dump($r);

$o = new UsingIndirect();
$r = $o->get_class_name_obj();
var_dump($r);
$r = $o->get_class_name_obj2();
var_dump($r);
}
