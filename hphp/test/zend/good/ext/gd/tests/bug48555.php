<?hh <<__EntryPoint>> function main(): void {
$cwd = dirname(__FILE__);
$font = "$cwd/Tuffy.ttf";
$box = imageftbbox(14.0, 0.0, $font, "Text without line-break");
//echo 'Top without line-break: ' . $box[7] . "\n";
$without_line_break = $box[7];
$box = imageftbbox(14.0, 0.0, $font, "Text with\nline-break\none more");
//echo 'Top with line-break: ' . $box[7] . "\n";
$with_line_break = $box[7];

var_dump($without_line_break);
var_dump($with_line_break);
if ($with_line_break==$without_line_break) {
  echo "with line break == without line break".PHP_EOL;
} else {
  echo "with line break != without line break".PHP_EOL;
}
}
