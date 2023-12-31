<?hh
class base {
    protected $p1 = 'base:1';
    public $p2 = 'base:2';
    public $p3 = 'base:3';
    public $p4 = 'base:4';
    public $p5 = 'base:5';
    private $p6 = 'base:6';
    public function __clone() :mixed{
    }
}

class test extends base {
    public $p1 = 'test:1';
    public $p3 = 'test:3';
    public $p4 = 'test:4';
    public $p5 = 'test:5';
    public function __clone() :mixed{
        $this->p5 = 'clone:5';
    }
}
<<__EntryPoint>> function main(): void {
$obj = new test;
$obj->p4 = 'A';
$copy = clone $obj;
echo "Object\n";
print_r($obj);
echo "Clown\n";
print_r($copy);
echo "Done\n";
}
