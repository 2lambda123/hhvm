<?hh

/*
 * Test for the datefmt_get_lenient & datefmt_set_lenient function
 */


function ut_main()
:mixed{

        $res_str = '';

    //Create
        $fmt = ut_datefmt_create( "en-US",  IntlDateFormatter::SHORT, IntlDateFormatter::SHORT , 'America/New_York', IntlDateFormatter::GREGORIAN );
        $res_str .= "\nIntlDateFormatter Created.\n";

        $resLenient1 = ut_datefmt_is_lenient( $fmt);
        $res_str .= "After call to get_lenient :  lenient= ";
    if( $resLenient1){
        $res_str .= "TRUE\n";
    }else{
        $res_str .= "FALSE\n";
    }

    //Set and test
        $res_str .= "--------------------\n";
        $isLenient = TRUE;
    $res_str .= "Setting IntlDateFormatter with lenient = ";
    if( $isLenient){
        $res_str .= "TRUE\n";
    }else{
        $res_str .= "FALSE\n";
    }
    ut_datefmt_set_lenient( $fmt , $isLenient );
    $resLenient = ut_datefmt_is_lenient( $fmt);
    $res_str .= "After call to is_lenient :  lenient= ";
    if( $resLenient){
        $res_str .= "TRUE\n";
    }else{
        $res_str .= "FALSE\n";
    }


    //Set and test
        $res_str .= "--------------------\n";
        $isLenient = FALSE;
    $res_str .= "Setting IntlDateFormatter with lenient =";
    if( $isLenient){
        $res_str .= "TRUE\n";
    }else{
        $res_str .= "FALSE\n";
    }
    ut_datefmt_set_lenient( $fmt , $isLenient);
    $resLenient = ut_datefmt_is_lenient( $fmt);
    $res_str .= "After call to is_lenient :  lenient= ";
    if( $resLenient){
        $res_str .= "TRUE\n";
    }else{
        $res_str .= "FALSE\n";
    }

        $res_str .= "--------------------\n";

        return $res_str;

}

<<__EntryPoint>> function main_entry(): void {
    include_once( 'ut_common.inc' );
    // Run the test
    ut_run();
}
