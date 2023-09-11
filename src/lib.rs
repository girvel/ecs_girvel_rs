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

struct Entity {
    name: Name,
    position: Position,
}

impl Component<Name> for Entity {
    fn get_component(&self) -> &Name {
        &self.name
    }
}

impl Component<Position> for Entity {
    fn get_component(&self) -> &Position {
        &self.position
    }
}

impl Entity {
    fn get<T>(&self) -> &T where Entity: Component<T> {
        Component::<T>::get_component(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let e = Entity {
            name: Name {name: "John Doe".to_string()},
            position: Position {x: 1, y: -1}
        };

        println!("{:?}", e.get::<Name>());
        assert_eq!(*e.get::<Name>(), e.name);
    }
}
