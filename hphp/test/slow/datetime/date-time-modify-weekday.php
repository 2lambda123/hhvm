<?hh

<<__EntryPoint>>
function main_date_time_modify_weekday() :mixed{
date_default_timezone_set('Asia/Taipei');

$dt = new DateTime('2012-01-02');
$dt->modify('3 tuesday');
var_dump($dt->format('Y-m-d'));
}
