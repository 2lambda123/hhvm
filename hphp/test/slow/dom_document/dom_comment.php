<?hh


<<__EntryPoint>>
function main_dom_comment() :mixed{
$dom = new DOMDocument('1.0', 'UTF-8');
$root = $dom->createElement('node');
$dom->appendChild($root);

$comment = $dom->createComment('comment');
$root->appendChild($comment);

echo $dom->saveXML();
}
