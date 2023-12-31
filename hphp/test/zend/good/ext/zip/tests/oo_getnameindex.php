<?hh

<<__EntryPoint>>
function main_entry(): void {
  $dirname = dirname(__FILE__) . '/';
  include $dirname . 'utils.inc';
  $file = sys_get_temp_dir().'/'.'__tmp_oo_rename2.zip';

  @unlink($file);

  $zip = new ZipArchive;
  if (!$zip->open($file, ZipArchive::CREATE)) {
  	exit('failed');
  }

  $zip->addFromString('entry1.txt', 'entry #1');
  $zip->addFromString('entry2.txt', 'entry #2');
  $zip->addFromString('dir/entry2d.txt', 'entry #2');

  if (!$zip->status == ZipArchive::ER_OK) {
  	echo "failed to write zip\n";
  }
  $zip->close();

  if (!$zip->open($file)) {
  	exit('failed');
  }


  var_dump($zip->getNameIndex(0));
  var_dump($zip->getNameIndex(1));
  var_dump($zip->getNameIndex(2));
  var_dump($zip->getNameIndex(3));
  @unlink($file);

  $zip->close();
}
