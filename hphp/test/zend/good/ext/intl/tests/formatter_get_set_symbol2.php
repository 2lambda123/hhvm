<?hh

/*
 * Get/set symbol.
 */


function ut_main()
:mixed{
    $longstr = str_repeat("blah", 10);
    $symbols = darray[
        'DECIMAL_SEPARATOR_SYMBOL' => varray[ NumberFormatter::DECIMAL_SEPARATOR_SYMBOL, '_._', 12345.123456, NumberFormatter::DECIMAL ],
        'GROUPING_SEPARATOR_SYMBOL' => varray[ NumberFormatter::GROUPING_SEPARATOR_SYMBOL, '_,_', 12345.123456, NumberFormatter::DECIMAL ],
        'PATTERN_SEPARATOR_SYMBOL' => varray[ NumberFormatter::PATTERN_SEPARATOR_SYMBOL, '_;_', 12345.123456, NumberFormatter::DECIMAL ],
        'PERCENT_SYMBOL' => varray[ NumberFormatter::PERCENT_SYMBOL, '_%_', 12345.123456, NumberFormatter::PERCENT ],
        'ZERO_DIGIT_SYMBOL' => varray[ NumberFormatter::ZERO_DIGIT_SYMBOL, '_ZD_', 12345.123456, NumberFormatter::DECIMAL ],
        'DIGIT_SYMBOL' => varray[ NumberFormatter::DIGIT_SYMBOL, '_DS_', 12345.123456, NumberFormatter::DECIMAL ],
        'MINUS_SIGN_SYMBOL' => varray[ NumberFormatter::MINUS_SIGN_SYMBOL, '_-_', -12345.123456, NumberFormatter::DECIMAL ],
        'PLUS_SIGN_SYMBOL' => varray[ NumberFormatter::PLUS_SIGN_SYMBOL, '_+_', 12345.123456, NumberFormatter::SCIENTIFIC ],
        'CURRENCY_SYMBOL' => varray[ NumberFormatter::CURRENCY_SYMBOL, '_$_', 12345.123456, NumberFormatter::CURRENCY ],
        'INTL_CURRENCY_SYMBOL' => varray[ NumberFormatter::INTL_CURRENCY_SYMBOL, '_$_', 12345.123456, NumberFormatter::CURRENCY ],
        'MONETARY_SEPARATOR_SYMBOL' => varray[ NumberFormatter::MONETARY_SEPARATOR_SYMBOL, '_MS_', 12345.123456, NumberFormatter::CURRENCY ],
        'EXPONENTIAL_SYMBOL' => varray[ NumberFormatter::EXPONENTIAL_SYMBOL, '_E_', 12345.123456, NumberFormatter::SCIENTIFIC ],
        'PERMILL_SYMBOL' => varray[ NumberFormatter::PERMILL_SYMBOL, '_PS_', 12345.123456, NumberFormatter::DECIMAL ],
        'PAD_ESCAPE_SYMBOL' => varray[ NumberFormatter::PAD_ESCAPE_SYMBOL, '_PE_', 12345.123456, NumberFormatter::DECIMAL ],
        'INFINITY_SYMBOL' => varray[ NumberFormatter::INFINITY_SYMBOL, '_IS_', 12345.123456, NumberFormatter::DECIMAL ],
        'NAN_SYMBOL' => varray[ NumberFormatter::NAN_SYMBOL, '_N_', 12345.123456, NumberFormatter::DECIMAL ],
        'SIGNIFICANT_DIGIT_SYMBOL' => varray[ NumberFormatter::SIGNIFICANT_DIGIT_SYMBOL, '_SD_', 12345.123456, NumberFormatter::DECIMAL ],
        'MONETARY_GROUPING_SEPARATOR_SYMBOL' => varray[ NumberFormatter::MONETARY_GROUPING_SEPARATOR_SYMBOL, '_MG_', 12345.123456, NumberFormatter::CURRENCY ],
    'MONETARY_GROUPING_SEPARATOR_SYMBOL-2' => varray[ NumberFormatter::MONETARY_GROUPING_SEPARATOR_SYMBOL, "&nbsp;", 12345.123456, NumberFormatter::CURRENCY ],
    'MONETARY_GROUPING_SEPARATOR_SYMBOL-3' => varray[ NumberFormatter::MONETARY_GROUPING_SEPARATOR_SYMBOL, $longstr, 12345.123456, NumberFormatter::CURRENCY ],
    ];

    $res_str = '';

    foreach( $symbols as $symb_name => $data )
    {
        list( $symb, $new_val, $number, $attr ) = $data;

        $fmt = ut_nfmt_create( 'en_US', $attr);

        $res_str .= "\nSymbol '$symb_name'\n";

        // Get original symbol value.
        $orig_val = ut_nfmt_get_symbol( $fmt, $symb );
        $res_str .= "Default symbol: [$orig_val]\n";

        // Set a new symbol value.
        $res_val = ut_nfmt_set_symbol( $fmt, $symb, $new_val );
        if( !$res_val )
            $res_str .= "set_symbol() error: " . ut_nfmt_get_error_message( $fmt ) . "\n";

        // Get the symbol value back.
        $new_val_check = ut_nfmt_get_symbol( $fmt, $symb );
        if( !$new_val_check )
            $res_str .= "get_symbol() error: " . ut_nfmt_get_error_message( $fmt ) . "\n";

        $res_str .= "New symbol: [$new_val_check]\n";

        // Check if the new value has been set.
        if( $new_val_check !== $new_val )
            $res_str .= "ERROR: New $symb_name symbol value has not been set correctly.\n";

        // Format the number using the new value.
        $s = ut_nfmt_format( $fmt, $number );
        $res_str .= "A number formatted with the new symbol: $s\n";

        // Restore attribute's symbol.
        ut_nfmt_set_symbol( $fmt, $symb, $orig_val );
    }
    $badvals = varray[2147483648, -2147483648, -1, 4294901761];
    foreach($badvals as $badval) {
        if(ut_nfmt_get_symbol( $fmt, 2147483648 ))  {
        $res_str .= "Bad value $badval should return false!\n";
        }
    }
    return $res_str;
}

<<__EntryPoint>> function main_entry(): void {
    include_once( 'ut_common.inc' );
    ut_run();
}
