<?hh


<<__EntryPoint>>
function main_file_hash() :mixed{
var_dump(md5_file(__DIR__.'/test_ext_file.txt'));
var_dump(sha1_file(__DIR__.'/test_ext_file.txt'));
}
