<?hh


<<__EntryPoint>>
function main_825() :mixed{
$vec1 = Vector {
11, 42, 73}
;
foreach ($vec1->items() as $x) {
  var_dump($x);
}
$mp1 = Map {
'a' => 1, 2 => 'b', 'z' => 9}
;
foreach ($mp1->items() as $t) {
  var_dump($t[0], $t[1]);
}
var_dump(new Vector($mp1->items()));
echo "------------------------\n";
$vec2 = Vector::fromItems($mp1->items());
var_dump($vec2);
$mp2 = Map::fromItems($mp1->items());
var_dump($mp2);
echo "------------------------\n";
$tuples = Vector {
Pair {
'a', 1}
, Pair {
2, 'b'}
, Pair {
'z', 9}
}
;
$mp3 = Map::fromItems($tuples);
var_dump($mp3);
}
