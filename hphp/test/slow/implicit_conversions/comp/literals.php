<?hh

class A {
  public $a = 5;
  public $b;
  function __construct($b = 'foo') {
    $this->b = $b;
  }
}

class B {
  public $a = 5;
  public $b;
  function __construct($b = 'foo') {
    $this->b = $b;
  }
}

class C {
  public $a = 5;
}

class DateTime1 implements DateTimeInterface {
  public $timestamp = 0;
  function __construct($timestamp) {
    $this->timestamp = $timestamp;
  }
  function getTimestamp() :mixed{
    if ($this->timestamp <= 0) {
      throw new Exception('sneaky');
    }
    return $this->timestamp;
  }
  function diff($dt, $absolute = null) :mixed{}
  function format($format) :mixed{}
  function getTimezone() :mixed{}
  function getOffset() :mixed{}
}

class DateTime2 implements DateTimeInterface {
  function getTimestamp() :mixed{
    return 100;
  }
  function diff($dt, $absolute = null) :mixed{}
  function format($format) :mixed{}
  function getTimezone() :mixed{}
  function getOffset() :mixed{}
}

function foo(): void {}
function rfoo<reify T>(): void {}

class Bar {
  public static function foo(): void {}
  public static function rfoo<reify T>(): void {}
}

function wrapped($f) :mixed{
  try {
    $f();
  } catch (Exception $e) {
    echo 'Threw: '.$e->getMessage()."\n";
  }
}

<<__EntryPoint>>
function main(): void {
  echo "\n\n<\n\n";
  less();
  echo "\n<=\n\n";
  lesseq();
  echo "\n>\n\n";
  gt();
  echo "\n>=\n\n";
  gte();
  echo "\n<=>\n\n";
  cmp();
}


function less(): void {
  echo "null < false\n";
  wrapped(() ==> null < false);
  echo "true < 0\n";
  wrapped(() ==> true < 0);
  echo "99 < 99.0\n";
  wrapped(() ==> 99 < 99.0);
  echo "empty string < NAN\n";
  wrapped(() ==> '' < NAN);
  echo "'0' < 'baz'\n";
  wrapped(() ==> "0" < 'baz');
  echo "'0' < 42\n";
  wrapped(() ==> "0" < 42);
  echo "'0bcd' < 1.234\n";
  wrapped(() ==> "0bcd" < 1.234);
  echo "'0bcd' < 'baz'\n";
  wrapped(() ==> "0bcd" < 'baz');
  echo "'0bcd' < 42\n";
  wrapped(() ==> "0bcd" < 42);
  echo "'0bcd' < 1.234\n";
  wrapped(() ==> "0bcd" < 1.234);

  echo "new A() < 42\n";
  wrapped(() ==> new A() < 42);
  echo "new A() < new A(new A())\n";
  wrapped(() ==> new A() < new A(new A()));
  echo "new B() < new C()\n";
  wrapped(() ==> new B() < new C());

  echo "DateTime1(1000) < new DateTime2()\n";
  wrapped(() ==> new DateTime1(1000) < new DateTime2());
  echo "DateTime1(1000) < 42\n";
  wrapped(() ==> new DateTime1(1000) < 42);

  echo "xml < 42\n";
  wrapped(() ==> simplexml_load_string("<root />")->unknown < 42);

  echo "Vector < Vector\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} < Vector {5, 6, 7, 8, 9});
  echo "Vector < 42\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} < 42);

  echo "Pair<string, string> < pair<string, int>\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} < Pair {'elem1', 42});
  echo "Pair < 42\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} < 42);

  echo "closure < 42\n";
  wrapped(
    () ==> function() {
      return 0;
    } <
      42,
  );

  echo "fun < 42\n";
  wrapped(() ==> foo<> < 42);
  echo "fun < 'baz'\n";
  wrapped(() ==> foo<> < 'baz');
  echo "rfun < 42\n";
  wrapped(() ==> rfoo<int> < 42);
  echo "class_meth < 42\n";
  wrapped(() ==> Bar::foo<> < 42);
  echo "class_meth < vec['foo', 'bar']\n";
  wrapped(() ==> Bar::foo<> < vec['foo', 'bar']);
  echo "class_meth < vec['Bar', 42]\n";
  wrapped(() ==> Bar::foo<> < vec['Bar', 42]);
  echo "rclass_meth < 42\n";
  wrapped(() ==> Bar::rfoo<int> < 42);

  echo "vec[99] < vec['foo']\n";
  wrapped(() ==> vec[99] < vec['foo']);
  echo "vec[vec[99]] < vec[vec['foo']]\n";
  wrapped(() ==> vec[vec[99]] < vec[vec['foo']]);
  echo "vec[42] < dict[0 => 42]\n";
  wrapped(() ==> vec[42] < dict[0 => 42]);
  echo "vec[42] < dict[0 => '42']\n";
  wrapped(() ==> vec[42] < dict[0 => '42']);
  echo "vec[42] < 42\n";
  wrapped(() ==> vec[42] < 42);
  echo "dict[0 => 42] < 42\n";
  wrapped(() ==> dict[0 => 42] > 42);
  echo "keyset[42] < 42\n";
  wrapped(() ==> keyset[42] < 42);

  $f1 = imagecreate(10, 10);
  echo "imagecreate(10, 10) < 42\n";
  wrapped(() ==> $f1 < 42);
  imagedestroy($f1);
}

function lesseq(): void {
  echo "null <= false\n";
  wrapped(() ==> null <= false);
  echo "true <= 0\n";
  wrapped(() ==> true <= 0);
  echo "99 <= 99.0\n";
  wrapped(() ==> 99 <= 99.0);
  echo "empty string <= NAN\n";
  wrapped(() ==> '' <= NAN);
  echo "'0' <= 'baz'\n";
  wrapped(() ==> "0" <= 'baz');
  echo "'0' <= 42\n";
  wrapped(() ==> "0" <= 42);
  echo "'0bcd' <= 1.234\n";
  wrapped(() ==> "0bcd" <= 1.234);
  echo "'0bcd' <= 'baz'\n";
  wrapped(() ==> "0bcd" <= 'baz');
  echo "'0bcd' <= 42\n";
  wrapped(() ==> "0bcd" <= 42);
  echo "'0bcd' <= 1.234\n";
  wrapped(() ==> "0bcd" <= 1.234);

  echo "new A() <= 42\n";
  wrapped(() ==> new A() <= 42);
  echo "new A() <= new A(new A())\n";
  wrapped(() ==> new A() <= new A(new A()));
  echo "new B() <= new C()\n";
  wrapped(() ==> new B() <= new C());

  echo "DateTime1(1000) <= new DateTime2()\n";
  wrapped(() ==> new DateTime1(1000) <= new DateTime2());
  echo "DateTime1(1000) <= 42\n";
  wrapped(() ==> new DateTime1(1000) <= 42);

  echo "xml <= 42\n";
  wrapped(() ==> simplexml_load_string("<root />")->unknown <= 42);

  echo "Vector <= Vector\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} <= Vector {5, 6, 7, 8, 9});
  echo "Vector <= 42\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} <= 42);

  echo "Pair<string, string> <= pair<string, int>\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} <= Pair {'elem1', 42});
  echo "Pair <= 42\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} <= 42);

  echo "closure <= 42\n";
  wrapped(
    () ==> function() {
      return 0;
    } <=
      42,
  );

  echo "fun <= 42\n";
  wrapped(() ==> foo<> <= 42);
  echo "fun <= 'baz'\n";
  wrapped(() ==> foo<> <= 'baz');
  echo "rfun <= 42\n";
  wrapped(() ==> rfoo<int> <= 42);
  echo "class_meth <= 42\n";
  wrapped(() ==> Bar::foo<> <= 42);
  echo "class_meth <= vec['foo', 'bar']\n";
  wrapped(() ==> Bar::foo<> <= vec['foo', 'bar']);
  echo "class_meth <= vec['Bar', 42]\n";
  wrapped(() ==> Bar::foo<> <= vec['Bar', 42]);
  echo "rclass_meth <= 42\n";
  wrapped(() ==> Bar::rfoo<int> <= 42);

  echo "vec[99] <= vec['foo']\n";
  wrapped(() ==> vec[99] <= vec['foo']);
  echo "vec[vec[99]] <= vec[vec['foo']]\n";
  wrapped(() ==> vec[vec[99]] <= vec[vec['foo']]);
  echo "vec[42] <= dict[0 => 42]\n";
  wrapped(() ==> vec[42] <= dict[0 => 42]);
  echo "vec[42] <= dict[0 => '42']\n";
  wrapped(() ==> vec[42] <= dict[0 => '42']);
  echo "vec[42] <= 42\n";
  wrapped(() ==> vec[42] <= 42);
  echo "dict[0 => 42] <= 42\n";
  wrapped(() ==> dict[0 => 42] > 42);
  echo "keyset[42] <= 42\n";
  wrapped(() ==> keyset[42] <= 42);

  $f1 = imagecreate(10, 10);
  echo "imagecreate(10, 10) <= 42\n";
  wrapped(() ==> $f1 <= 42);
  imagedestroy($f1);
}

function gt(): void {
  echo "null > false\n";
  wrapped(() ==> null > false);
  echo "true > 0\n";
  wrapped(() ==> true > 0);
  echo "99 > 99.0\n";
  wrapped(() ==> 99 > 99.0);
  echo "empty string > NAN\n";
  wrapped(() ==> '' > NAN);
  echo "'0' > 'baz'\n";
  wrapped(() ==> "0" > 'baz');
  echo "'0' > 42\n";
  wrapped(() ==> "0" > 42);
  echo "'0bcd' > 1.234\n";
  wrapped(() ==> "0bcd" > 1.234);
  echo "'0bcd' > 'baz'\n";
  wrapped(() ==> "0bcd" > 'baz');
  echo "'0bcd' > 42\n";
  wrapped(() ==> "0bcd" > 42);
  echo "'0bcd' > 1.234\n";
  wrapped(() ==> "0bcd" > 1.234);

  echo "new A() > 42\n";
  wrapped(() ==> new A() > 42);
  echo "new A() > new A(new A())\n";
  wrapped(() ==> new A() > new A(new A()));
  echo "new B() > new C()\n";
  wrapped(() ==> new B() > new C());

  echo "DateTime1(1000) > new DateTime2()\n";
  wrapped(() ==> new DateTime1(1000) > new DateTime2());
  echo "DateTime1(1000) > 42\n";
  wrapped(() ==> new DateTime1(1000) > 42);

  echo "xml > 42\n";
  wrapped(() ==> simplexml_load_string("<root />")->unknown > 42);

  echo "Vector > Vector\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} > Vector {5, 6, 7, 8, 9});
  echo "Vector > 42\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} > 42);

  echo "Pair<string, string> > pair<string, int>\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} > Pair {'elem1', 42});
  echo "Pair > 42\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} > 42);

  echo "closure > 42\n";
  wrapped(
    () ==> function() {
      return 0;
    } >
      42,
  );

  echo "fun > 42\n";
  wrapped(() ==> foo<> > 42);
  echo "fun > 'baz'\n";
  wrapped(() ==> foo<> > 'baz');
  echo "rfun > 42\n";
  wrapped(() ==> rfoo<int> > 42);
  echo "class_meth > 42\n";
  wrapped(() ==> Bar::foo<> > 42);
  echo "class_meth > vec['foo', 'bar']\n";
  wrapped(() ==> Bar::foo<> > vec['foo', 'bar']);
  echo "class_meth > vec['Bar', 42]\n";
  wrapped(() ==> Bar::foo<> > vec['Bar', 42]);
  echo "rclass_meth > 42\n";
  wrapped(() ==> Bar::rfoo<int> > 42);

  echo "vec[99] > vec['foo']\n";
  wrapped(() ==> vec[99] > vec['foo']);
  echo "vec[vec[99]] > vec[vec['foo']]\n";
  wrapped(() ==> vec[vec[99]] > vec[vec['foo']]);
  echo "vec[42] > dict[0 => 42]\n";
  wrapped(() ==> vec[42] > dict[0 => 42]);
  echo "vec[42] > dict[0 => '42']\n";
  wrapped(() ==> vec[42] > dict[0 => '42']);
  echo "vec[42] > 42\n";
  wrapped(() ==> vec[42] > 42);
  echo "dict[0 => 42] > 42\n";
  wrapped(() ==> dict[0 => 42] > 42);
  echo "keyset[42] > 42\n";
  wrapped(() ==> keyset[42] > 42);

  $f1 = imagecreate(10, 10);
  echo "imagecreate(10, 10) > 42\n";
  wrapped(() ==> $f1 > 42);
  imagedestroy($f1);
}

function gte(): void {
  echo "null >= false\n";
  wrapped(() ==> null >= false);
  echo "true >= 0\n";
  wrapped(() ==> true >= 0);
  echo "99 >= 99.0\n";
  wrapped(() ==> 99 >= 99.0);
  echo "empty string >= NAN\n";
  wrapped(() ==> '' >= NAN);
  echo "'0' >= 'baz'\n";
  wrapped(() ==> "0" >= 'baz');
  echo "'0' >= 42\n";
  wrapped(() ==> "0" >= 42);
  echo "'0bcd' >= 1.234\n";
  wrapped(() ==> "0bcd" >= 1.234);
  echo "'0bcd' >= 'baz'\n";
  wrapped(() ==> "0bcd" >= 'baz');
  echo "'0bcd' >= 42\n";
  wrapped(() ==> "0bcd" >= 42);
  echo "'0bcd' >= 1.234\n";
  wrapped(() ==> "0bcd" >= 1.234);

  echo "new A() >= 42\n";
  wrapped(() ==> new A() >= 42);
  echo "new A() >= new A(new A())\n";
  wrapped(() ==> new A() >= new A(new A()));
  echo "new B() >= new C()\n";
  wrapped(() ==> new B() >= new C());

  echo "DateTime1(1000) >= new DateTime2()\n";
  wrapped(() ==> new DateTime1(1000) >= new DateTime2());
  echo "DateTime1(1000) >= 42\n";
  wrapped(() ==> new DateTime1(1000) >= 42);

  echo "xml >= 42\n";
  wrapped(() ==> simplexml_load_string("<root />")->unknown >= 42);

  echo "Vector >= Vector\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} >= Vector {5, 6, 7, 8, 9});
  echo "Vector >= 42\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} >= 42);

  echo "Pair<string, string> >= pair<string, int>\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} >= Pair {'elem1', 42});
  echo "Pair >= 42\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} >= 42);

  echo "closure >= 42\n";
  wrapped(
    () ==> function() {
      return 0;
    } >=
      42,
  );

  echo "fun >= 42\n";
  wrapped(() ==> foo<> >= 42);
  echo "fun >= 'baz'\n";
  wrapped(() ==> foo<> >= 'baz');
  echo "rfun >= 42\n";
  wrapped(() ==> rfoo<int> >= 42);
  echo "class_meth >= 42\n";
  wrapped(() ==> Bar::foo<> >= 42);
  echo "class_meth >= vec['foo', 'bar']\n";
  wrapped(() ==> Bar::foo<> >= vec['foo', 'bar']);
  echo "class_meth >= vec['Bar', 42]\n";
  wrapped(() ==> Bar::foo<> >= vec['Bar', 42]);
  echo "rclass_meth >= 42\n";
  wrapped(() ==> Bar::rfoo<int> >= 42);

  echo "vec[99] >= vec['foo']\n";
  wrapped(() ==> vec[99] >= vec['foo']);
  echo "vec[vec[99]] >= vec[vec['foo']]\n";
  wrapped(() ==> vec[vec[99]] >= vec[vec['foo']]);
  echo "vec[42] >= dict[0 => 42]\n";
  wrapped(() ==> vec[42] >= dict[0 => 42]);
  echo "vec[42] >= dict[0 => '42']\n";
  wrapped(() ==> vec[42] >= dict[0 => '42']);
  echo "vec[42] >= 42\n";
  wrapped(() ==> vec[42] >= 42);
  echo "dict[0 => 42] >= 42\n";
  wrapped(() ==> dict[0 => 42] > 42);
  echo "keyset[42] >= 42\n";
  wrapped(() ==> keyset[42] >= 42);

  $f1 = imagecreate(10, 10);
  echo "imagecreate(10, 10) >= 42\n";
  wrapped(() ==> $f1 >= 42);
  imagedestroy($f1);
}

function cmp(): void {

  echo "null <=> false\n";
  wrapped(() ==> null <=> false);
  echo "true <=> 0\n";
  wrapped(() ==> true <=> 0);
  echo "99 <=> 99.0\n";
  wrapped(() ==> 99 <=> 99.0);
  echo "empty string <=> NAN\n";
  wrapped(() ==> '' <=> NAN);
  echo "'0' <=> 'baz'\n";
  wrapped(() ==> "0" <=> 'baz');
  echo "'0' <=> 42\n";
  wrapped(() ==> "0" <=> 42);
  echo "'0bcd' <=> 1.234\n";
  wrapped(() ==> "0bcd" <=> 1.234);
  echo "'0bcd' <=> 'baz'\n";
  wrapped(() ==> "0bcd" <=> 'baz');
  echo "'0bcd' <=> 42\n";
  wrapped(() ==> "0bcd" <=> 42);
  echo "'0bcd' <=> 1.234\n";
  wrapped(() ==> "0bcd" <=> 1.234);

  echo "new A() <=> 42\n";
  wrapped(() ==> new A() <=> 42);
  echo "new A() <=> new A(new A())\n";
  wrapped(() ==> new A() <=> new A(new A()));
  echo "new B() <=> new C()\n";
  wrapped(() ==> new B() <=> new C());

  echo "DateTime1(1000) <=> new DateTime2()\n";
  wrapped(() ==> new DateTime1(1000) <=> new DateTime2());
  echo "DateTime1(1000) <=> 42\n";
  wrapped(() ==> new DateTime1(1000) <=> 42);

  echo "xml <=> 42\n";
  wrapped(() ==> simplexml_load_string("<root />")->unknown <=> 42);

  echo "Vector <=> Vector\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} <=> Vector {5, 6, 7, 8, 9});
  echo "Vector <=> 42\n";
  wrapped(() ==> Vector {0, 1, 2, 3, 4} <=> 42);

  echo "Pair<string, string> <=> pair<string, int>\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} <=> Pair {'elem1', 42});
  echo "Pair <=> 42\n";
  wrapped(() ==> Pair {'elem1', 'elem2'} <=> 42);

  echo "closure <=> 42\n";
  wrapped(
    () ==> function() {
      return 0;
    } <=>
      42,
  );

  echo "fun <=> 42\n";
  wrapped(() ==> foo<> <=> 42);
  echo "fun <=> 'baz'\n";
  wrapped(() ==> foo<> <=> 'baz');
  echo "rfun <=> 42\n";
  wrapped(() ==> rfoo<int> <=> 42);
  echo "class_meth <=> 42\n";
  wrapped(() ==> Bar::foo<> <=> 42);
  echo "class_meth <=> vec['foo', 'bar']\n";
  wrapped(() ==> Bar::foo<> <=> vec['foo', 'bar']);
  echo "class_meth <=> vec['Bar', 42]\n";
  wrapped(() ==> Bar::foo<> <=> vec['Bar', 42]);
  echo "rclass_meth <=> 42\n";
  wrapped(() ==> Bar::rfoo<int> <=> 42);

  echo "vec[99] <=> vec['foo']\n";
  wrapped(() ==> vec[99] <=> vec['foo']);
  echo "vec[vec[99]] <=> vec[vec['foo']]\n";
  wrapped(() ==> vec[vec[99]] <=> vec[vec['foo']]);
  echo "vec[42] <=> dict[0 => 42]\n";
  wrapped(() ==> vec[42] <=> dict[0 => 42]);
  echo "vec[42] <=> dict[0 => '42']\n";
  wrapped(() ==> vec[42] <=> dict[0 => '42']);
  echo "vec[42] <=> 42\n";
  wrapped(() ==> vec[42] <=> 42);
  echo "dict[0 => 42] <=> 42\n";
  wrapped(() ==> dict[0 => 42] > 42);
  echo "keyset[42] <=> 42\n";
  wrapped(() ==> keyset[42] <=> 42);

  $f1 = imagecreate(10, 10);
  echo "imagecreate(10, 10) <=> 42\n";
  wrapped(() ==> $f1 <=> 42);
  imagedestroy($f1);
}
