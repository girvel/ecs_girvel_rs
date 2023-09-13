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
    fn get_component(&self) -> Rc<T>;
}

trait Entity {
    fn get<T>(&self) -> Rc<T> where Self: Component<T>;
}

impl<E: ?Sized> Entity for E {
    fn get<T>(&self) -> Rc<T> where Self: Component<T> {
        Component::<T>::get_component(self)
    }
}

// macro_rules!

macro_rules! entity {
    ($name:ident: $($component:ident),*) => {
        #[allow(non_snake_case)]
        struct $name {
            $( $component: Rc<$component> ),*
        }

        $(impl Component<$component> for $name {
            fn get_component(&self) -> Rc<$component> {
                self.$component.clone()
            }
        })*
    };
}

entity!(Example: Name, Position);


entity!(NameSystemSubject: Name);

impl NameSystemSubject {
    fn from<T>(entity: T) -> Self where T: Component<Name> {
        Self {
            Name: entity.get::<Name>(),
        }
    }
}

struct NameSystem {
    subjects: Vec<NameSystemSubject>,
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
    fn raise(subject: &NameSystemSubject) {
        println!("{}", subject.get::<Name>().name)
    }

    fn update(&self) {
        for subject in &self.subjects {
            Self::raise(subject)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let e = Example {
            Name: Rc::new(Name {name: "John Doe".to_string()}),
            Position: Rc::new(Position {x: 1, y: -1}),
        };

        println!("{:?}", e.get::<Name>());
        assert_eq!(e.get::<Name>().name, "John Doe");

        let mut name_system = NameSystem::default();
        name_system.subjects.push(NameSystemSubject::from(e));
        name_system.update()
    }
}
