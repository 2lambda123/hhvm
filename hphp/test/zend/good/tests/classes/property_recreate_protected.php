<?hh
class C {
    protected $p = 'test';
    function unsetProtected() :mixed{
        unset($this->p);
    }
    function setProtected() :mixed{
        $this->p = 'changed';
    }
}

class D extends C {
    function setP() :mixed{
        $this->p = 'changed in D';
    }
}
<<__EntryPoint>> function main(): void {
$d = new D;
echo "Unset and recreate a protected property from property's declaring class scope:\n";
$d->unsetProtected();
$d->setProtected();
var_dump($d);

echo "\nUnset and recreate a protected property from subclass:\n";
$d = new D;
$d->unsetProtected();
$d->setP();
var_dump($d);

echo "\nUnset a protected property, and attempt to recreate it outside of scope (expected failure):\n";
$d->unsetProtected();
$d->p = 'this will fail';
var_dump($d);
}
