use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

pub struct Container<'de, T> where T: Serialize, T: Deserialize<'de> {
    buf: &'de [u8],
    _phantom0: PhantomData<T>,
}

impl<'de, T> Container<'de, T> where T: Serialize, T: Deserialize<'de> {
    pub fn new(buf: &'de [u8]) -> Self {
        Self { buf, _phantom0: PhantomData  }
    }

    pub fn read(self) -> Result<T, flexbuffers::DeserializationError> {
        flexbuffers::from_slice(self.buf)
    }
}

#[cfg(test)]
mod test {
    use flexbuffers::FlexbufferSerializer;
    use serde::{Deserialize, Serialize};

    use super::Container;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
    struct Person {
        pub age: u32,
        pub name: String,
    }

    #[test]
    fn constructor() {
        let person = Person { age: 32, name: "Henry".to_owned() };
        let mut serializer = FlexbufferSerializer::new();

        person.serialize(&mut serializer).unwrap();

        let container: Container<Person> = Container::new(serializer.view());
        let deserialized = container.read().unwrap();

        assert_eq!(deserialized, person);
    }
}