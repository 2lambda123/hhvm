<?hh


<<__EntryPoint>>
function main_typecheck() :mixed{
$not_ready = \HH\Client\typecheck(__DIR__.'/hh_client_notready');
apc_delete('__systemlib__hh_client_time');
var_dump($not_ready);
echo "\n";

$clean = \HH\Client\typecheck(__DIR__.'/hh_client_clean');
apc_delete('__systemlib__hh_client_time');
var_dump($clean);
echo json_encode($clean, JSON_FB_FORCE_PHP_ARRAYS);
echo "\n";

$error = \HH\Client\typecheck(__DIR__.'/hh_client_error');
apc_delete('__systemlib__hh_client_time');
var_dump($error);
echo json_encode($error, JSON_FB_FORCE_PHP_ARRAYS);
echo "\n";

$noclient = \HH\Client\typecheck(__DIR__.'/this_file_does_not_exist');
apc_delete('__systemlib__hh_client_time');
var_dump($noclient);
echo json_encode($noclient, JSON_FB_FORCE_PHP_ARRAYS);
echo "\n";

$not_ready->triggerError();
$clean->triggerError(E_WARNING);
$error->triggerError(E_WARNING);
$error->triggerError(E_RECOVERABLE_ERROR);
}
