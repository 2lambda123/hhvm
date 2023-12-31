<?hh
/* Prototype: bool symlink ( string $target, string $link );
   Description: creates a symbolic link to the existing target with the specified name link

   Prototype: bool is_link ( string $filename );
   Description: Tells whether the given file is a symbolic link.

   Prototype: bool link ( string $target, string $link );
   Description: Create a hard link

   Prototype: int linkinfo ( string $path );
   Description: Gets information about a link
*/

/* Variation 7 : Create soft/hard link to itself */
// temp file used
<<__EntryPoint>> function main(): void {

$dir = sys_get_temp_dir().'/'.'symlink_link_linkinfo_is_link_variation7';
$filename = "$dir/symlink_link_linkinfo_is_link_variation7.tmp";
// link name used
$linkname = "$dir/symlink_link_linkinfo_is_link_link_variation7.tmp";
// temp dirname used
$dirname = "$dir/home/test";
mkdir($dirname, 0755, true);

// create file
$fp = fopen($filename, "w");
fclose($fp);

echo "*** Create soft link to file and then to itself ***\n";
// create soft link to $filename
var_dump( symlink($filename, $linkname) );
// create another link to $linkname
var_dump( symlink($linkname, $linkname) );
// delete link
unlink($linkname);

echo "\n*** Create soft link to directory and then to itself ***\n";
// create soft link to $dirname
var_dump( symlink($dirname, $linkname) );
// create another link to $dirname
var_dump( symlink($linkname, $linkname) );
// delete link
unlink($linkname);

echo "\n*** Create hard link to file and then to itself ***\n";
// create hard link to $filename
var_dump( link($filename, $linkname) );
// create another link to $linkname
var_dump( link($linkname, $linkname) );
// delete link
unlink($linkname);

echo "Done\n";

unlink($filename);
rmdir("$dir/home/test");
rmdir("$dir/home");
rmdir($dir);
}
