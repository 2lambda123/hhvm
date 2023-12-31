<?hh
/* Prototype:
 *  array array_column(array $input, mixed $column_key[, mixed $index_key]);
 * Description:
 *  Returns an array containing all the values from
 *  the specified "column" in a two-dimensional array.
 */
class Foo
{
    public function __toString()
:mixed    {
        return 'last_name';
    }
}
class Bar
{
    public function __toString()
:mixed    {
        return 'first_name';
    }
}
<<__EntryPoint>> function main(): void {
echo "*** Testing array_column() : basic functionality ***\n";
/* Array representing a possible record set returned from a database */
$records = varray[
    darray[
        'id' => 1,
        'first_name' => 'John',
        'last_name' => 'Doe'
    ],
    darray[
        'id' => 2,
        'first_name' => 'Sally',
        'last_name' => 'Smith'
    ],
    darray[
        'id' => 3,
        'first_name' => 'Jane',
        'last_name' => 'Jones'
    ]
];

echo "-- first_name column from recordset --\n";
var_dump(array_column($records, 'first_name'));

echo "-- id column from recordset --\n";
var_dump(array_column($records, 'id'));

echo "-- last_name column from recordset, keyed by value from id column --\n";
var_dump(array_column($records, 'last_name', 'id'));

echo "-- last_name column from recordset, keyed by value from first_name column --\n";
var_dump(array_column($records, 'last_name', 'first_name'));

echo "\n*** Testing multiple data types ***\n";
$fh = fopen(__FILE__, 'r', true);
$values = varray[
    darray[
        'id' => 1,
        'value' => new stdClass
    ],
    darray[
        'id' => 2,
        'value' => 34.2345
    ],
    darray[
        'id' => 3,
        'value' => true
    ],
    darray[
        'id' => 4,
        'value' => false
    ],
    darray[
        'id' => 5,
        'value' => null
    ],
    darray[
        'id' => 6,
        'value' => 1234
    ],
    darray[
        'id' => 7,
        'value' => 'Foo'
    ],
    darray[
        'id' => 8,
        'value' => $fh
    ]
];
var_dump(array_column($values, 'value'));
var_dump(array_column($values, 'value', 'id'));

echo "\n*** Testing numeric column keys ***\n";
$numericCols = varray[
    varray['aaa', '111'],
    varray['bbb', '222'],
    darray[0 => 'ccc', 1 => '333', -1 => 'ddd']
];
var_dump(array_column($numericCols, 1));
var_dump(array_column($numericCols, 1, 0));
var_dump(array_column($numericCols, 1, 0.123));
var_dump(array_column($numericCols, 1, -1));

echo "\n*** Testing failure to find specified column ***\n";
var_dump(array_column($numericCols, 2));
var_dump(array_column($numericCols, 'foo'));
var_dump(array_column($numericCols, 0, 'foo'));
var_dump(array_column($numericCols, 3.14));

echo "\n*** Testing single dimensional array ***\n";
$singleDimension = varray['foo', 'bar', 'baz'];
var_dump(array_column($singleDimension, 1));

echo "\n*** Testing columns not present in all rows ***\n";
$mismatchedColumns = varray[
    darray['a' => 'foo', 'b' => 'bar', 'e' => 'bbb'],
    darray['a' => 'baz', 'c' => 'qux', 'd' => 'aaa'],
    darray['a' => 'eee', 'b' => 'fff', 'e' => 'ggg'],
];
var_dump(array_column($mismatchedColumns, 'c'));
var_dump(array_column($mismatchedColumns, 'c', 'a'));
var_dump(array_column($mismatchedColumns, 'a', 'd'));
var_dump(array_column($mismatchedColumns, 'a', 'e'));
var_dump(array_column($mismatchedColumns, 'b'));
var_dump(array_column($mismatchedColumns, 'b', 'a'));

echo "\n*** Testing use of object converted to string ***\n";
$f = new Foo();
$b = new Bar();
var_dump(array_column($records, $f));
var_dump(array_column($records, $f, $b));

echo "Done\n";
}
