// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use std::convert::{TryInto, TryFrom};
use std::error;
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Your task is to complete this implementation
// in order for the line `let p = Person::try_from("Mark,20")` to compile
// and return an Ok result of inner type Person.
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object

#[derive(PartialEq, Debug)]
enum PersonError {
    Parse,
}

impl fmt::Display for PersonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &dyn error::Error).description())
    }
}

impl error::Error for PersonError {
    fn description(&self) -> &str {
        match *self {
            PersonError::Parse => "Parse",
        }
    }
}


impl TryFrom<&str> for Person {
    type Error = Box<dyn error::Error>;
    fn try_from(s: &str) -> Result<Self, Box<dyn error::Error>> {
        let parts: Vec<_> = s.split(",").collect();
        if parts.len() != 2 { return Err(Box::new(PersonError::Parse)) }
        let name = parts[0].to_string();
        let age = parts[1].parse::<usize>()?;
        Ok(Person {
            name,
            age,
        })
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::try_from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Result<Person, _> = "Gerald,70".try_into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::try_from("");
        assert!(p.is_err());
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::try_from("Mark,20");
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    #[should_panic]
    fn test_panic_empty_input() {
        let p: Person = "".try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_panic_bad_age() {
        let p = Person::try_from("Mark,twenty").unwrap();
    }
}