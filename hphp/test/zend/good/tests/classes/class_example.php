<?hh

/* pretty nifty object oriented code! */

class user {
  public $first_name,$family_name,$address,$phone_num;
  function display()
:mixed  {
    echo "User information\n";
    echo "----------------\n\n";
    echo "First name:\t  ".$this->first_name."\n";
    echo "Family name:\t  ".$this->family_name."\n";
    echo "Address:\t  ".$this->address."\n";
    echo "Phone:\t\t  ".$this->phone_num."\n";
    echo "\n\n";
  }
  function initialize($first_name,$family_name,$address,$phone_num)
:mixed  {
    $this->first_name = $first_name;
    $this->family_name = $family_name;
    $this->address = $address;
    $this->phone_num = $phone_num;
  }
}

function test($u) :mixed{
  /* one can pass classes as arguments */
  $u->display();
  $t = $u;
  $t->address = "New address...";
  return $t;  /* and also return them as return values */
}

<<__EntryPoint>> function main(): void {
$user1 = new user;
$user2 = new user;

$user1->initialize("Zeev","Suraski","Ben Gourion 3, Kiryat Bialik, Israel","+972-4-8713139");
$user2->initialize("Andi","Gutmans","Haifa, Israel","+972-4-8231621");
$user1->display();
$user2->display();

$tmp = test($user2);
$tmp->display();
}
