<?hh <<__EntryPoint>> function main(): void {
$client = new SoapClient(dirname(__FILE__).'/bug28985.wsdl', darray['trace'=>1]);
var_dump($client->__getTypes());
}
