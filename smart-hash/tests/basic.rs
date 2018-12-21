extern crate smart_hash;
use smart_hash::get_matching;
use smart_hash::traits::{SmartHash,SmartHashOpt, SmartHashSet};

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
// make a derive here.
pub struct Tester {
    value1 : u8,
    value2 : bool,
    value3 : String,
}

// part of the derive macro
#[derive(Hash, Debug)]
pub struct TesterOpt {
    value1 : Option<u8>,
    value2 : Option<bool>,
    value3 : Option<String>,
}
// part of derive macro
impl Default for TesterOpt {
    fn default() -> TesterOpt {
        TesterOpt {
            value1 : None,
            value2 : None,
            value3 : None,
        }
    }
}

// part of the derive macro
impl PartialEq for TesterOpt {
    fn eq(&self, other: &TesterOpt) -> bool {
        (self.value1 == other.value1 || self.value1.is_none() || other.value1.is_none()) &&
        (self.value2 == other.value2 || self.value2.is_none() || other.value2.is_none()) &&
        (self.value3 == other.value3 || self.value3.is_none() || other.value3.is_none())
    }
}
// part of the derive macro
impl Eq for TesterOpt {}

// part of the derive macro
impl SmartHash for Tester {
    type option = TesterOpt; 
    
    fn into_option(&self) -> TesterOpt {
        TesterOpt {
            value1 : Some(self.value1),
            value2 : Some(self.value2),
            value3 : Some(self.value3.to_string()),
        }
    }
}

// part of the derive macro
impl SmartHashOpt for TesterOpt { }

#[test]
fn test_macro() {
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

    // style 1
    let result = get_matching!(testers,value1,10);
    assert_eq!(result.unwrap().len(),2);

    // style 2
    let result = get_matching!(testers,value1 == 10);
    assert_eq!(result.unwrap().len(),2);

    // style 3
    let result = get_matching!(testers where value1 is 10, value2 is true);
    assert_eq!(result.unwrap().len(),1);
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