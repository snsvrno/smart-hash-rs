pub mod traits;

#[macro_export]
macro_rules! get_matching {

    // basic initial format
    // get_matching!(object, key, var, key2, var2);
    ($obj:ident,$($key:ident,$val:expr),+) => ({
            let mut empty_opt = $obj.get_none_default();
            
            $(empty_opt.$key = Some($val);)*

            $obj.get_matching(empty_opt)
    });

    // more obvious what is going on in this format
    // get_matching!(object, key == var, key2 == var2);
    ($obj:ident,$($key:ident == $val:expr),+) => ({            
        get_matching!($obj,$($key,$val),+)
    });

    // a more of a query format
    // get_matching!(object where key is var, key2 is var2);
    ($obj:ident where $($key:ident is $val:expr) +) => ({
        get_matching!($obj,$($key,$val),+)
    });
}