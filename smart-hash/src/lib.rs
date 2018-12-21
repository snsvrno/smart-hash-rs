pub mod traits;

#[macro_export]
macro_rules! get_matching {
    ($obj:ident,$($key:ident,$val:expr),+) => (

        // sorta working, but we need to figure out how to get `TesterOpt`
        // from the `ident` `$obj`, or some nicer way to do this, so its cleaner
        // for the operator
        $obj.get_matching(TesterOpt{
            $($key : Some($val),)+
            ..Default::default()
        })
    )
}