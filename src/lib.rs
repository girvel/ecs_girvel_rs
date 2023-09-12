use std::rc::Rc;

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

impl<E: ?Sized> Entity for E {
    fn get<T>(&self) -> &T where Self: Component<T> {
        Component::<T>::get_component(self)
    }
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
    };
}

entity!(Example: Name, Position);

struct NameSystem {
    subjects: Vec<Rc<dyn Component<Name>>>,
}

impl Default for NameSystem {
    fn default() -> Self {
        Self {
            subjects: vec![],
        }
    }
}

impl NameSystem {
    // TODO maybe remove dynamic dispatch futher down the line?
    fn raise<S>(subject: Rc<S>) where S: Component<Name> + ?Sized {
        println!("{}", (*subject).get::<Name>().name)
    }

    fn update(&self) {
        for subject in &self.subjects {
            Self::raise(subject.clone())
        }
    }
}

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

        let mut name_system = NameSystem::default();
        name_system.subjects.push(Rc::new(e));
        name_system.update()
    }
}
