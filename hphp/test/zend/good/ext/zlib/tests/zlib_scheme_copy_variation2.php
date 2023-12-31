<?hh <<__EntryPoint>> function main(): void {
$org_data = <<<EOT
uncompressed contents of 004.txt.gz is:
When you're taught through feelings
Destiny flying high above
all I know is that you can realize it
Destiny who cares
as it turns around
and I know that it descends down on me
EOT;

$inputFileName = sys_get_temp_dir().'/'.'org';
$outputFileName = sys_get_temp_dir().'/'.'tmp';

file_put_contents($inputFileName, $org_data);

$srcFile = $inputFileName;
$destFile = "compress.zlib://$outputFileName";
copy($srcFile, $destFile);

$h = gzopen($outputFileName, 'r');
$copied_data = gzread($h, 4096);
gzclose($h);

//gzopen can read compressed and uncompressed so we
//also need to look for the magic number (x1f x8b) to prove it
//was compressed.
$h = fopen($outputFileName, 'r');
$magic = fread($h, 2);
fclose($h);

if ($org_data == $copied_data && bin2hex($magic) === '1f8b') {
   echo "OK: Copy identical\n";
}
else {
   echo "FAILED: Copy not identical\n";
}
unlink($inputFileName);
unlink($outputFileName);
echo "===DONE===\n";
}
