// #[macro_export] macro_rules! entity {
//     ($name:ident: $($component:ident),*) => {
//         #[allow(non_snake_case)]
//         struct $name {
//             $( $component: std::rc::Rc<$component> ),*
//         }
//
//         $(impl ecs_girvel::Component<$component> for $name {
//             fn get_component(&self) -> std::rc::Rc<$component> {
//                 self.$component.clone()
//             }
//         })*
//     };
// }
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn entity(_: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);

    let name = &item_struct.ident;
    let fields = &item_struct.fields;

    return quote! {
        #item_struct
    }.into();
}