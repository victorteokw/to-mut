use test;
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;

#[derive(ToMut)]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn test_it_mutates() {
    let person = Person { name: "John".to_owned(), age: 20 };
    let mut mut_person = person.to_mut();
    mut_person.name = "Peter".to_owned();
    mut_person.age = 30;
    assert_eq!(person.age, 30);
    assert_eq!(person.name, "Peter".to_owned());
}
