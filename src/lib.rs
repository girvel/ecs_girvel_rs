use std::rc::Rc;

pub trait Component<T> {
    fn get_component(&self) -> Rc<T>;
}

pub trait Entity {
    fn get<T>(&self) -> Rc<T> where Self: Component<T>;
}

impl<E: ?Sized> Entity for E {
    fn get<T>(&self) -> Rc<T> where Self: Component<T> {Component::<T>::get_component(self)}
}

#[macro_export] macro_rules! entity {
    ($name:ident: $($component:ident),*) => {
        #[allow(non_snake_case)]
        struct $name {
            $( $component: std::rc::Rc<$component> ),*
        }

        $(impl ecs_girvel::Component<$component> for $name {
            fn get_component(&self) -> std::rc::Rc<$component> {
                self.$component.clone()
            }
        })*
    };
}