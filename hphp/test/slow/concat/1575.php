<?hh


<<__EntryPoint>>
function main_1575() :mixed{
echo '\\' . "\n";
echo '\'' . "\n";
echo '\\' . '\'' . "\n";
echo '\\' . "'" . "\n";
echo "\\" . "'" . "\n";
echo "\101" . "\n";
echo "\\" . "101" . "\n";
echo "\1" . "101" . "\n";
echo "\01" . "01" . "\n";
echo "\01" . "g" . "\n";
echo "\1" . "g" . "\n";
echo "\011" . "01" . "\n";
echo "\0111" . "01" . "\n";
echo "\x" . "1" . "\n";
echo "\x1" . "1" . "\n";
echo "\x11" . "1" . "\n";
echo "\x111" . "1" . "\n";
echo "\x1111" . "1" . "\n";
echo "\x11111" . "1" . "\n";
echo "\777777" . "7" . "\n";
echo "\0777777" . "7" . "\n";
echo "\00777777" . "7" . "\n";
echo "\0077\"7777" . "7" . "\n";
echo "\0077\\7777" . "7" . "\n";
echo "\0077\a7777" . "7" . "\n";
echo "\0077\b7777" . "7" . "\n";
echo "\0077\f7777" . "7" . "\n";
echo "\0077\n7777" . "7" . "\n";
echo "\0077\r7777" . "7" . "\n";
echo "\0077\t7777" . "7" . "\n";
echo "\0077\v7777" . "7" . "\n";
echo "\0077\07777" . "7" . "\n";
echo "\0077\'7777" . "7" . "\n";
}
