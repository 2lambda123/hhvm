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

/* Variation 8 : Create soft/hard link to different directory */
/* creating link to a file in different dir with the same name as the file */
<<__EntryPoint>> function main(): void {
echo "\n*** Create hard link in different directory with same filename ***\n";
// temp file used

$filename = sys_get_temp_dir().'/'.'symlink_link_linkinfo_is_link_variation8.tmp';
// temp link name used
$dirname = sys_get_temp_dir().'/'.'symlink_link_linkinfo_is_link1_variation8';
mkdir($dirname);
$linkname = "symlink_link_linkinfo_is_link_variation8.tmp";
// create temp file
$fp = fopen($filename, "w");
fclose($fp);

var_dump( link($filename, $dirname."/") ); // this fails indicating file exists
// ok, creates "symlink_link_linkinfo_is_link1_variation8/symlink_link_linkinfo_is_link_variation8.tmp" link
var_dump( link($filename, $dirname."/".$linkname) );  // this works fine
// delete link
unlink($dirname."/".$linkname);
// delete temp file
unlink($filename);
// delete temp dir
rmdir($dirname);

echo "\n*** Create soft link in different directory with same filename ***\n";
$filename = sys_get_temp_dir().'/'.'symlink_link_linkinfo_is_link_variation8.tmp';
// temp link name used
$dirname = sys_get_temp_dir().'/'.'symlink_link_linkinfo_is_link1_variation8';
mkdir($dirname);
$linkname = "symlink_link_linkinfo_is_link_variation8.tmp";
// create temp file
$fp = fopen($filename, "w");
fclose($fp);

var_dump( symlink($filename, $dirname."/") ); // this fails indicating file exists
// ok, creates "symlink_link_linkinfo_is_link1_variation8/symlink_link_linkinfo_is_link_variation8.tmp" link
var_dump( symlink($filename, $dirname."/".$linkname) );  // this works fine
// delete link
unlink($dirname."/".$linkname);
// delete temp file
unlink($filename);
// delete temp dir
rmdir($dirname);

echo "Done\n";
}
