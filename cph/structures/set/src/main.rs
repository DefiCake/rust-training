use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Person {
  name: String,
  age: u8,
}

fn main() {
  let mut people: HashSet<Person> = HashSet::new();

  let alice: Person = Person { name: "alice".to_string(), age: 20 };
  let bob: Person = Person { name: "bob".to_string(), age: 20 };
  let carol: Person = Person { name: "carol".to_string(), age: 20 };
  let dave: Person = Person { name: "dave".to_string(), age: 20 };

  people.insert(alice.clone());
  people.insert(bob.clone());
  people.insert(carol.clone());

  dbg!(people.contains(&alice));
  dbg!(people.contains(&bob));
  dbg!(people.contains(&carol)); // ^ true
  dbg!(people.contains(&dave)); // false
}
