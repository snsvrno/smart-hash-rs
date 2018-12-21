# SmartHash
Smart Hash is a collection of traits that allows you to use a `HashSet` with a `struct` of your design to make quick and easy queries.

## What is it?

_smart-hash_ adds a trait which makes it easy to search for partial matches inside of a _HashSet_. Think of it as querying your _HashSet_ for matching data.

### Limitations

- Only can match on equality (no lt, gt, etc ...)

## Minimum Example

A quick 'get it working' example to show what it does, and how to set it up.

```rust
#[macro_use] extern crate smart-hash;
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
    
    // not using the macro access method (same result as below)
    let people_25 = people.get_matching(PeopleOpt{
        name : None,
        age : Some(25),
        height : None,
    });

    // using the macro access method (same result as above)
    let people_25 = get_matching!(people,age == 25);

    // looks at the results
    if let Some(people) in people_25 {
        for p in people {
            // should only print out the people who are 25 (where age == 25)
            println!({},p.name);
        }
    }
}
```
## Whats Included?

### smart-hash

The _smart-hash_ crate includes the following public traits.

#### SmartHash

The ***SmartHash*** trait makes the struct for partial matching. It requires a second struct to exist (the option) and a function `into_option` to create a _option struct_ with the values in the original struct.

Assume your data structure looks like this.

```rust
pub Struct Car {
    color : Color,
    make : String,
    year : Date,
}
```

In order to implement ***SmartHash*** you would need to add the following. 

**NOTE:** all this is automatically done for you if you use `#[derive(SmartHash)]` from [smart-hash-derive](#smart-hash-derive).

```rust
// the options struct
pub Struct CarOpt {
    color : Option<Color>,
    make : Option<String>,
    year : Option<Date>,
}

// SmartHash requires Default to be implemented
impl Default for CarOpt {
    fn default() -> CarOpt {
        CarOpt { 
            color: None, 
            make: None, 
            year:None 
        }
    }
}

// implement SmartHash
impl SmartHash for Car {
    type option = CarOpt;

    fn into_option(&self) -> CarOpt {
        CarOpt {
            color: Some(self.color),
            make : Some(self.make),
            year : Some(self.year),
        }
    }
}
```

#### SmartHashOpt

The new option struct needs some empty traits implemented aswell..

**NOTE:** all this is automatically done for you if you use `#[derive(SmartHash)]` from [smart-hash-derive](#smart-hash-derive).

```rust

impl SmartHashOpt for CarOpt { }

// allows the partial matching
impl Eq for CarOpt {}
impl PartialEq for CarOpt {
    fn eq(&self, other: &CarOpt) -> bool {
        (self.color == other.color || self.color.is_none() || other.color.is_none()) &&
        (self.make == other.make || self.make.is_none() || other.make.is_none()) &&
        (self.year == other.year || self.year.is_none() || other.year.is_none())
    }
}

```

#### SmartHashSet

A trait that has a default implementation for all structs that implement _SmartHash_ that are inside a _HashSet_. This adds an additional function to the _HashSet_ do you can do fuzzy searches / partial queries.

```rust
// in our example, the signature looks like this
fn get_matching<'a>(&'a self : HashSet<Car>, query : CarOpt) -> Option<Vec<&'a Car>>
```

#### get_matching!() Macro

A macro is also included to make searching easier and cleaner. Once you mark the extern with `#[macro_use]` you can then use the following macro.

```rust
// basic signature descriptions
macro get_matching!(object, key, value, k2, v2, ...) -> Option<Vec<object_member>>;
macro get_matching!(object, key == value, k2 == v2, ...) -> Option<Vec<object_member>>;
macro get_matching!(object where key is value, k2 is v2, ...) -> Option<Vec<object_member>>;
```

### smart-hash-derive

Crate including the procedural macro. There is nothing to do here except `#[macro_use]` the extern and then use the macro.

**NOTE:** the other derives, _Hash_, _Eq_, _PartialEq_ are requirements for HashSet and required as well.

```rust
#[derive(Hash, Eq, PartialEq, SmartHash)]
pub struct Person {
    name : String,
    age : u8,
    country : String,
}
```

## Organization

### smart-hash

The _smart-hash_ crate has everything you need, but makes it complicated. You'll have to implement a lot. You can look at the [test](smart-hash/tests/basic.rs) to see what needs to be written. Most of it is boiler plate and copy and paste. I'd recommend using the `derive` macro.

### smart-hash-derive

The _smart-hash-derive_ crate is a procedural macro that will derive _SmartHash_ and its children traits automatically so you don't have to worry about a thing.

## Future

- Allow for all comparision operators in the macro: lt, gt, <=, >=, <>, etc ....

