#![recursion_limit="128"]
// for the traits
extern crate smart_hash;

// for the macro
#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate proc_macro2; use proc_macro2::{Ident,Span};
extern crate syn; use syn::Type;

#[proc_macro_derive(SmartHash)]
pub fn smart_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let ast = syn::parse(input).unwrap();

    impl_smart_hash(&ast)
}

fn impl_smart_hash(ast : &syn::DeriveInput) -> proc_macro::TokenStream {
    //* creates the 

    let name = &ast.ident;
    let name_opt = Ident::new(&format!("{}Opt",name),Span::call_site());

    let mut m_prime : Vec<Ident> = Vec::new();
    let mut t_prime : Vec<Type> = Vec::new();

    if let syn::Data::Struct(ref datastruct) = ast.data {
        if let syn::Fields::Named(ref fields) = datastruct.fields {
            for p in &fields.named {
                if let Some(ref ident) = p.ident {
                    m_prime.push(ident.clone());
                }
                t_prime.push(p.ty.clone());
            }
        }
    }

    // garbage because of issue #8 in the github repo for 'quote-rs'
    // can't use the same variable more than once in a expansion, i.e. #(#a #a)*
    let ref m = m_prime;
    let m2 = m;
    let m3 = m;
    let m4 = m; 

    // not needed, but for consistency.
    let ref t = t_prime;

    let gen = quote! {
        use smart_hash::traits::SmartHashSet;

        #[derive(Hash, Debug)]
        pub struct #name_opt {
            #(#m : Option<#t>),*
        }

        impl Eq for #name_opt { }
        impl smart_hash::traits::SmartHashOpt for #name_opt { }
        impl PartialEq for #name_opt {
            fn eq(&self, other : &#name_opt) -> bool {
                #( ((self.#m == other.#m2) || other.#m3.is_none() || self.#m4.is_none()) ) && *
            }
        }

        impl Default for #name_opt {
            fn default() -> #name_opt {
                #name_opt {
                    #( #m : None ),*
                }
            }
        }

        impl smart_hash::traits::SmartHash for #name {
            type option = #name_opt;

            fn into_option(&self) -> #name_opt {
                #name_opt {
                    #(#m : Some(self.#m2.clone())),*
                }
            }
        }
    };

    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
