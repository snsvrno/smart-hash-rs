extern crate smart_hash;
use smart_hash::traits::{SmartHash,SmartHashOpt, SmartHashSet};

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Tester {
    value1 : u8,
    value2 : bool,
    value3 : String,
}

#[derive(Hash, Debug)]
pub struct TesterOpt {
    value1 : Option<u8>,
    value2 : Option<bool>,
    value3 : Option<String>,
}

impl PartialEq for TesterOpt {
    fn eq(&self, other: &TesterOpt) -> bool {
        (self.value1 == other.value1 || self.value1.is_none() || other.value1.is_none()) &&
        (self.value2 == other.value2 || self.value2.is_none() || other.value2.is_none()) &&
        (self.value3 == other.value3 || self.value3.is_none() || other.value3.is_none())
    }
}
impl Eq for TesterOpt {}

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

impl SmartHashOpt for TesterOpt { }

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

    println!("Found: {:?}",matches);
    assert!(false);


}