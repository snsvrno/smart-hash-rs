pub mod traits;

#[macro_export]
macro_rules! get_matching {
    ($obj:ident,$($key:ident,$val:expr),+) => (
        {
            let mut empty_opt = $obj.get_none_default();
            
            $(empty_opt.$key = Some($val);)*

            $obj.get_matching(empty_opt)
        }
    );

    ($obj:ident,$($key:ident == $val:expr),+) => (
        {
            get_matching!($obj,$($key,$val),+)
        }
    );
}