#[derive(PartialEq, Eq, Debug)]
struct Name {
    name: String,
}

#[derive(PartialEq, Eq, Debug)]
struct Position {
    x: i8,
    y: i8,
}

trait Component<T> {
    fn get_component(&self) -> &T;
}

struct Example {
    name: Name,
    position: Position,
}

impl Component<Name> for Example {
    fn get_component(&self) -> &Name {
        &self.name
    }
}

impl Component<Position> for Example {
    fn get_component(&self) -> &Position {
        &self.position
    }
}

// impl Entity {
//     fn get<T>(&self) -> &T where Entity: Component<T> {
//         Component::<T>::get_component(self)
//     }
// }

trait Entity {
    fn get<T>(&self) -> &T where Self: Component<T>;
}

impl Entity for Example {
    fn get<T>(&self) -> &T where Self: Component<T> {
        Component::<T>::get_component(self)
    }
}

// macro_rules! entity {
//     ($name:expr, $component1:expr, $component2:expr) => {
//
//     }
// }
//
// entity!(Example, Name, Position);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let e = Example {
            name: Name {name: "John Doe".to_string()},
            position: Position {x: 1, y: -1}
        };

        println!("{:?}", e.get::<Name>());
        assert_eq!(*e.get::<Name>(), e.name);
    }
}
