<?hh

class A {
    public $a = 1;
    static public $A = 2;

    private $b = 3;
    static private $B = 4;

    protected $c = 5;
    static protected $C = 6;

    public function __construct() {
        var_dump(get_class_vars('A'));
    }

    static public function test() :mixed{
        var_dump(get_class_vars('A'));
    }
}
<<__EntryPoint>> function main(): void {
var_dump(get_class_vars('A'));

new A;

var_dump(A::test());
}
