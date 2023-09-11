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

trait Entity {
    fn get<T>(&self) -> &T where Self: Component<T>;
}

// macro_rules!

macro_rules! entity {
    ($name:ident: $($component:ident),*) => {
        #[allow(non_snake_case)]
        struct $name {
            $( $component: $component ),*
        }

        $(impl Component<$component> for $name {
            fn get_component(&self) -> &$component {
                &self.$component
            }
        })*

        impl Entity for $name {
            fn get<T>(&self) -> &T where Self: Component<T> {
                Component::<T>::get_component(self)
            }
        }
    };
}

entity!(Example: Name, Position);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let e = Example {
            Name: Name {name: "John Doe".to_string()},
            Position: Position {x: 1, y: -1},
        };

        println!("{:?}", e.get::<Name>());
        assert_eq!(e.get::<Name>().name, "John Doe");
    }
}
