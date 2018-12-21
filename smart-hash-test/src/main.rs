extern crate smart_hash;
use smart_hash::{get_matching,SmartHash};
use smart_hash::traits::SmartHashSet;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, SmartHash)]
pub struct Person {
    name : String,
    age : u8,
    country : String,
}

fn main() {
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
    println!("{} people found, should be 2",people_set_one.unwrap().len());

    let people_set_two = get_matching!(people,country,"USA".to_string());
    println!("{} people found, should be 3",people_set_two.unwrap().len());
    
}