<?hh
/* Prototype: int filegroup ( string $filename )
 * Description: Returns the group ID of the file, or FALSE in case of an error.
 */
<<__EntryPoint>> function main(): void {
echo "*** Testing filegroup(): basic functionality ***\n"; 

echo "-- Testing with the file or directory created by owner --\n";

var_dump( filegroup(__FILE__) );
var_dump( filegroup(".") );
var_dump( filegroup("./..") );

/* Newly created files and dirs */
$file_name = sys_get_temp_dir().'/'.'filegroup_basic.tmp';
$file_handle = fopen($file_name, "w");

$string = "Hello, world\n1234\n123Hello";
fwrite($file_handle, $string);
var_dump( filegroup($file_name) );
fclose($file_handle);

$dir_name = sys_get_temp_dir().'/'.'filegroup_basic';
mkdir($dir_name);
var_dump( filegroup($dir_name) );

echo "\n-- Testing with the standard file or directory --\n";
var_dump( filegroup("/etc/passwd") );
var_dump( filegroup("/etc") );
var_dump( filegroup("/") );

echo "\n*** Done ***\n";

unlink($file_name);
rmdir($dir_name);
}
