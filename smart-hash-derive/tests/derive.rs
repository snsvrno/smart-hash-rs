extern crate smart_hash;
#[macro_use] extern crate smart_hash_derive;

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, SmartHash)]
pub struct Tester {
    value1: u8,
    value2: bool,
    value3: String,
}

#[test]
fn test_usage() {
    let mut testers : HashSet<Tester> = HashSet::new();

    testers.insert(Tester {
        value1 : 10,
        value2 : false,
        value3 : "Some String".to_string(),
    });

    testers.insert(Tester {
        value1 : 10,
        value2 : true,
        value3 : "Some String".to_string(),
    });

    let matches = testers.get_matching(TesterOpt {
        value1 : Some(10),
        value2 : None,
        value3 : None,
    });

    assert_eq!(matches.unwrap().len(),2);

    let matches = testers.get_matching(TesterOpt {
        value1 : None,
        value2 : Some(false),
        value3 : None,
    });

    assert_eq!(matches.unwrap().len(),1);

    let matches = testers.get_matching(TesterOpt {
        value1 : Some(23),
        value2 : Some(false),
        value3 : None,
    });

    assert!(matches.is_none());

}