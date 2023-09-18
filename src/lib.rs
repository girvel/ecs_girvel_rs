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
use syn::Fields;

#[proc_macro_attribute]
pub fn entity(_: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);

    let name = &item_struct.ident;
    let components = match &item_struct.fields {
        Fields::Unnamed(f) => &f.unnamed,
        _ => panic!("Expected a struct with named fields"),
    }.iter()
        .map(|f|);

    let components: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            quote! { #ident }
        })
        .collect();

    return quote! {
        #item_struct;


    }.into();
}