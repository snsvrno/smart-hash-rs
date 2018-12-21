# SmartHash
Smart Hash is a collection of traits that allows you to use a `HashSet` with a `struct` of your design to make quick and easy queries.

## Minimum Example

```rust
extern crate smart-hash;
#[macro_use] extern crate smart-hash-derive;
use smart_hash::traits::SmartHashSet;

#[derive(SmartHash)]
pub struct Person {
    name : String,
    age : u8,
    height: u8,
}

pub fn main() {
    let people : HashSet<Person> = HashSet::New();
    
    // do something here to add a bunch of people into it....
    
    let people_25 = people.get_matching(PeopleOpt{
        name : None,
        age : Some(25),
        height : None,
    });

    if let Some(people) in people_25 {
        for p in people {
            // should only print out the people who are 25 (where age == 25)
            println!({},p.name);
        }
    }
}
```

## Future

- Allow for expressions when pattern matching, currently we can do this `get_matching!(hashmap,param1 = value1);` but it would be nicer to be able to do this `get_matching!(hashmap,param1 <= value1, param2 == value2);'

