<?hh

function f1() :mixed{
  $a = Vector {
11, 22}
;
  $b = Vector {
33, 44}
;
  $a->addAll($b);
  var_dump($a);
}
function f2() :mixed{
  $a = Vector {
11, 22}
;
  $b = Map {
'a' => 33, 'b' => 44}
;
  $a->addAll($b);
  var_dump($a);
}
function f3() :mixed{
  $a = Map {
'a' => 11, 'b' => 22}
;
  $b = Vector {
Pair {
'e', 33}
, Pair {
'f', 44}
}
;
  $a->addAll($b);
  var_dump($a);
}
function f4() :mixed{
  $a = Map {
'a' => 11, 'b' => 22}
;
  $b = Map {
'c' => Pair {
'e', 33}
, 'd' => Pair {
'f', 44}
}
;
  $a->addAll($b);
  var_dump($a);
}

<<__EntryPoint>>
function main_2194() :mixed{
f1();
f2();
f3();
f4();
}
