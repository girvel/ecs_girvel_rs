use std::rc::Rc;
use ecs_girvel::entity;
use ecs_girvel_common::{Component, Entity};

#[derive(PartialEq, Eq, Debug)]
struct Name {
    name: String,
}

#[derive(PartialEq, Eq, Debug)]
struct Position {
    x: i8,
    y: i8,
}

// entity!(NameSystemSubject: Name);
#[entity]
struct NameSystemSubject(Name);

// impl NameSystemSubject {
//     fn from<T: Component<Name> + ?Sized>(entity: &T) -> Self {
//         Self {
//             Name: entity.get::<Name>(),
//         }
//     }
// }
//
// struct NameSystem {
//     subjects: Vec<NameSystemSubject>,
// }
//
// impl Default for NameSystem {
//     fn default() -> Self {
//         Self {
//             subjects: vec![],
//         }
//     }
// }
//
// impl NameSystem {
//     fn raise(subject: &NameSystemSubject) {
//         println!("{}", subject.get::<Name>().name)
//     }
//
//     fn update(&self) {
//         for subject in &self.subjects {
//             Self::raise(subject)
//         }
//     }
// }
//
// entity!(Example: Name, Position);


#[test]
fn it_works() {
    // let e = Example {
    //     Name: Rc::new(Name { name: "John Doe".to_string() }),
    //     Position: Rc::new(Position { x: 1, y: -1 }),
    // };
    //
    // assert_eq!(e.get::<Name>().name, "John Doe");
    //
    // let mut name_system = NameSystem::default();
    // name_system.subjects.push(NameSystemSubject::from(&e));
    //
    // name_system.update();
}