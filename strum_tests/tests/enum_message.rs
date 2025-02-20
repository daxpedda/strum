use strum::EnumMessage;

#[derive(Debug, Eq, PartialEq, EnumMessage)]
enum Pets {
    #[strum(message = "I'm a dog")]
    Dog,
    #[strum(message = "I'm a cat")]
    #[strum(detailed_message = "I'm a very exquisite striped cat")]
    Cat,
    #[strum(detailed_message = "My fish is named Charles McFish")]
    Fish,
    Bird,
    #[strum(disabled)]
    Hamster,
}

#[test]
fn simple_message() {
    assert_eq!("I'm a dog", (Pets::Dog).get_message().unwrap());
    assert_eq!("I'm a dog", (Pets::Dog).get_detailed_message().unwrap());
}

#[test]
fn detailed_message() {
    assert_eq!("I'm a cat", (Pets::Cat).get_message().unwrap());
    assert_eq!(
        "I'm a very exquisite striped cat",
        (Pets::Cat).get_detailed_message().unwrap()
    );
}

#[test]
fn only_detailed_message() {
    assert_eq!(None, (Pets::Fish).get_message());
    assert_eq!(
        "My fish is named Charles McFish",
        (Pets::Fish).get_detailed_message().unwrap()
    );
}

#[test]
fn no_message() {
    assert_eq!(None, (Pets::Bird).get_message());
    assert_eq!(None, (Pets::Bird).get_detailed_message());
}

#[test]
fn disabled_message() {
    assert_eq!(None, (Pets::Hamster).get_message());
    assert_eq!(None, (Pets::Hamster).get_detailed_message());
}

#[derive(Debug, Eq, PartialEq, EnumMessage)]
#[strum(serialize_all = "kebab_case")]
enum Brightness {
    DarkBlack,
    Dim {
        glow: usize,
    },
    #[strum(serialize = "bright")]
    BrightWhite,
}

#[test]
fn get_serializations() {
    assert_eq!(
        vec!["dark-black"],
        (Brightness::DarkBlack).get_serializations()
    );
    assert_eq!(
        vec!["dim"],
        (Brightness::Dim { glow: 1 }).get_serializations()
    );
    assert_eq!(
        vec!["bright"],
        (Brightness::BrightWhite).get_serializations()
    );
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Eq, PartialEq, EnumMessage)]
    #[strum(crate = "nested::module::strum")]
    enum Pets {
        #[strum(message = "I'm a dog")]
        Dog,
        #[strum(message = "I'm a cat")]
        #[strum(detailed_message = "I'm a very exquisite striped cat")]
        Cat,
        #[strum(detailed_message = "My fish is named Charles McFish")]
        Fish,
        Bird,
        #[strum(disabled)]
        Hamster,
    }

    assert_eq!("I'm a dog", (Pets::Dog).get_message().unwrap());
    assert_eq!("I'm a dog", (Pets::Dog).get_detailed_message().unwrap());
}
