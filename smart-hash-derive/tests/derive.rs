#[macro_use] extern crate smart_hash;
#[macro_use] extern crate smart_hash_derive;
use smart_hash::traits::SmartHashSet;

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, SmartHash)]
pub struct Tester {
    value1: u8,
    value2: bool,
    value3: String,
}

#[derive(Hash, Eq, PartialEq, Debug, SmartHash)]
pub struct Person {
    name : String,
    age : u8,
    country : String,
}

#[test]
fn test_macro() {
    let mut people : HashSet<Person> = HashSet::new();

    people.insert(Person{
        name : "Jim Jones".to_string(),
        age : 45,
        country : "USA".to_string(),
    });
    people.insert(Person{
        name : "Linda Hammersmith".to_string(),
        age : 78,
        country : "USA".to_string(),
    });
    people.insert(Person{
        name : "Pecante Jones".to_string(),
        age : 23,
        country : "USA".to_string(),
    });
    people.insert(Person{
        name : "George Linda".to_string(),
        age : 23,
        country : "Canada".to_string(),
    });
    
    let people_set_one = get_matching!(people,age == 23);
    assert!(people_set_one.unwrap().len() == 2);

    
    let people_set_two = get_matching!(people,country,"USA".to_string());
    assert!(people_set_two.unwrap().len() == 3);
    
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