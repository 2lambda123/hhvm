<?hh

// define a simple generator that returns a series of consecutive values

function series($start, $end)
:AsyncGenerator<mixed,mixed,void>{
    for ($i = $start; $i <= $end; ++$i)
    {
        yield $i;
    }
}
//*/

///*
// define a generator that returns sucessive lines from a file of text

function getTextFileLines($filename)
:AsyncGenerator<mixed,mixed,void>{
    $infile = fopen($filename, 'r');
    if ($infile == FALSE)
    {
        // handle file-open failure
    }

    try
    {
        while ($textLine = fgets($infile))  // while not EOF
        {
//          echo "len before rtrim: " . + strlen($textLine) . "\n";
            $textLine = rtrim($textLine, "\r\n");   // strip off line terminator
//          echo "len after rtrim:  " . + strlen($textLine) . "\n";
            yield $textLine;
        }
    }
    finally
    {
        fclose($infile);
    }
}
//*/

///*
// See if return is allowed in any form in a generator function
// also yield on its own + yield in an expression

function f()
:AsyncGenerator<mixed,mixed,void>{
//  if (0)      // even if the yield is never executed, can't have return with value
    for ($i = 1; $i <= 3; ++$i)
    {
//      yield;              // PHP5: uses next-available int key and value NULL
                            // HHVM: Fatal error: syntax error, unexpected ';'
        yield $i;
//      return;
//      return NULL;
//      $x = (yield $i);    // PHP5: yield produces a NULL result
                            // HHVM: Fatal error: syntax error, unexpected T_YIELD
//      var_dump($x);
    }
//  return;         // PHP5: OK
                    // HHVM: Fatal error: Cannot mix 'return' and 'yield' in the same function
//  return NULL;    // PHP5: OK; it is like return;, which returns NULL implicitly
                    // HHVM: Fatal error: Cannot mix 'return' and 'yield' in the same function
    $n = NULL;
//  return $n;      // error: Generators cannot return values using "return"
//  return 0;       // error: Generators cannot return values using "return"
}
//*/

///*
// define a simple generator that returns key/value pairs

function series2($start, $end, $keyPrefix = "")
:AsyncGenerator<mixed,mixed,void>{
    for ($i = $start; $i <= $end; ++$i)
    {
//      yield;                          // default key is int 0,1,2... and value NULL
        yield $keyPrefix . $i => $i;    // specify a key/value pair
//      $x = (yield $keyPrefix . $i => $i); // PHP5: yield produces a NULL result
                                        // HHVM: Fatal error: syntax error, unexpected T_YIELD
//      var_dump($x);
    }
}
//*/
<<__EntryPoint>>
function main_entry(): void {
  error_reporting(-1);

  ///*
  foreach (series(1, 5) as $key => $val)
  {
      echo "key: $key, value: $val\n";
  }
  //*/

  ///*

  // Just what does a function generator return?

  echo "===========================\n";
  $a = series(5, 15);
  var_dump($a);   // PHP5 and HHVM: type Generator
  echo "This type " . (($a is Generator)
                       ? "is"
                       : "is not")
                    . " an instance of Generator\n";

  // $serialString = serialize($a);   // Uncaught exception

  echo "===========================\n";

  foreach (getTextFileLines(__DIR__."/Testfile.txt") as $line)
  {
      echo ">$line<\n";
  }
  echo "===========================\n";

  foreach (f() as $key => $val)
  {
      echo "key: $key, value: $val\n";
  }
  echo "===========================\n";

  foreach (series2(1, 5, "X") as $key => $val)
  {
      echo "key: $key, value: $val\n";
  //  echo ($val == NULL) ? "value is NULL\n" : "value is not NULL\n";
  }
}
