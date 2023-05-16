<?hh
/*
 * Prototype: int fileinode ( string $filename );
 * Description: Returns the inode number of the file, or FALSE in case of an error.
 */
/* Creating soft and hard links to a file and applying fileinode() on links */
<<__EntryPoint>> function main(): void {

fclose( fopen(sys_get_temp_dir().'/'.'fileinode_variation1.tmp', "w")) ;

echo "*** Testing fileinode() with links ***\n";
/* With symlink */
symlink(
  sys_get_temp_dir().'/'.'fileinode_variation1.tmp',
  sys_get_temp_dir().'/'.'fileinode_variation1_symlink.tmp'
);
var_dump( fileinode(sys_get_temp_dir().'/'.'fileinode_variation1_symlink.tmp')) ; //expected true
clearstatcache();

/* With hardlink */
link(
  sys_get_temp_dir().'/'.'fileinode_variation1.tmp',
  sys_get_temp_dir().'/'.'fileinode_variation1_link.tmp');
var_dump( fileinode(sys_get_temp_dir().'/'.'fileinode_variation1_link.tmp')) ;  // expected: true
clearstatcache();

echo "\n*** Done ***";

unlink(sys_get_temp_dir().'/'.'fileinode_variation1_symlink.tmp');
unlink(sys_get_temp_dir().'/'.'fileinode_variation1_link.tmp');
unlink(sys_get_temp_dir().'/'.'fileinode_variation1.tmp');
}
