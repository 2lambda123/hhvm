<?hh


<<__EntryPoint>>
function main_559() :mixed{
$a=1;
$a='t';
 $b = $a;
 $a[10]= 'AB';
 var_dump($a);
 var_dump($b);
}
