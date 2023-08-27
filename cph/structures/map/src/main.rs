use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Person {
  name: String,
  age: u8,
}

fn main() {
  let alice: Person = Person { name: "alice".to_string(), age: 20 };
  let bob: Person = Person { name: "bob".to_string(), age: 20 };
  let carol: Person = Person { name: "carol".to_string(), age: 20 };

  let people: HashMap<String, Person> = HashMap::from([
    (alice.name.to_string().clone(), alice.clone()),
    (bob.name.to_string().clone(), bob.clone()),
    (carol.name.to_string().clone(), carol.clone()),
  ]);

  dbg!(people.get("alice"));
  dbg!(people.get("bob"));
  dbg!(people.get("carol"));
  dbg!(people.get("dave")); // None
}
