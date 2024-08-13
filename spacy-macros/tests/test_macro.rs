use spacy_macros_derive::add_uuid;
use uuid::Uuid;

#[add_uuid]
struct Pancake {
    _name: String,
}

#[test]
fn it_works() {
    let p = Pancake::new(String::from("Egg and Bacon"));

    if let Some(u) = p.uuid {
        println!("{}", u);
    }

    let _ = p.clone();
}
