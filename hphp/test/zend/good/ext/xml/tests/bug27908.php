<?hh

function x_default_handler($xp,$data)
:mixed{
    echo "x_default_handler $data\n";
}
<<__EntryPoint>> function main(): void {
$xp = xml_parser_create();
xml_set_default_handler($xp,x_default_handler<>);
xml_parse($xp, '<root></root>',TRUE);
xml_parser_free($xp);
echo "Done\n";
}
