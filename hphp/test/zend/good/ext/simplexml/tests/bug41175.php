<?hh
<<__EntryPoint>> function main(): void {
$xml = new SimpleXMLElement("<img></img>");
$xml->addAttribute("src", "foo");
$xml->addAttribute("alt", "");
echo $xml->asXML();

echo "===DONE===\n";
}
