use proc_macro2::{Literal, Punct, Span, TokenStream, TokenTree};
use syn::{Attribute, DeriveInput, Token, Type, parse::Parser, parse_macro_input, punctuated::Punctuated};
use quote::{ToTokens, TokenStreamExt, quote};


pub(crate) fn get_parameters(structure: &DeriveInput) -> TokenStream {
    structure.generics.params.clone().iter().map(|parameter: &syn::GenericParam|{
        let mut token = match parameter {
            syn::GenericParam::Const(con) => {
                con.ident.clone().to_token_stream()
            }
            syn::GenericParam::Lifetime(lifetime) => {
                lifetime.lifetime.clone().to_token_stream()
            }
            syn::GenericParam::Type(ty) => {
                ty.ident.clone().to_token_stream()
            }
        }.into_iter().collect::<Vec<_>>();
        
        let punct = proc_macro2::TokenTree::Punct(Punct::new(',', proc_macro2::Spacing::Alone));
        token.push(punct);
        token
        
    }).flatten().collect::<TokenStream>()
}

pub(crate) fn get_fields(structure: &DeriveInput) -> (Vec<TokenTree>, bool, bool) {
    let (val, is_tuple) = match &structure.data {
        syn::Data::Struct(data) => {
            let fields = data.fields.iter();
            (fields, data.fields.iter().next().expect("Expected fields").ident.is_none())
        }
        syn::Data::Union(_) => {
            panic!("Expected struct, found union");
        }
        syn::Data::Enum(_) => {
            panic!("Expected struct, found enum");
        }
    };
    let is_first_element_array = if let Type::Array(_) = val.clone().next().unwrap().ty {
        true
    } else {
        false
    };
    if is_tuple {
        (val.enumerate().map(|(i, _)|{
            TokenTree::Literal(Literal::usize_unsuffixed(i))
        }).collect::<Vec<_>>(), is_tuple, is_first_element_array)
    } else {
        (val.map(|field|{
            TokenTree::Ident(field.ident.clone().unwrap())
        }).collect::<Vec<_>>(), is_tuple, is_first_element_array)
    }
}

fn to_mul_function(fields: &Vec<TokenTree>, name: &syn::Ident, info: &MatrixMulInfo) -> TokenStream {
    let mut vec_fields = vec![];
    let output = info.output.clone();
    let ty = info.ty.clone();
    
    println!("Self: {}x{}", fields.len(), info.columns.len());
    // Rows come from Self
    // Columns come from Rhs
    for self_column in info.columns.clone() {
        let scalars = info.rows.clone().iter().map(|rhs_column|{
            // Amount of repeats on sum should equal amount of columns
            // in Self / rows in Rhs
            let fields = fields.clone();
            quote! {
                #(self.#fields.#rhs_column*rhs.#self_column.#fields)+*,
            }
        }).collect::<TokenStream>();
        vec_fields.push(quote! {
            <#output<T> as Matrix>::Vector::new(#scalars)
        });
    }
    let mul_function: TokenStream = quote! {
        impl<T: Ring + core::ops::Mul<Output = T>+ core::ops::Add<Output = T>> core::ops::Mul<#ty<T>> for #name<T> {
            type Output = #output<T>;
            fn mul(self, rhs: #ty<T>) -> Self::Output {
                #output::new(
                    #(#vec_fields),*
                )
            }
        }
    };
    mul_function
}
struct MatrixMulInfo {
    ty: syn::Ident,
    output: syn::Ident,
    columns: Vec<TokenTree>,
    rows: Vec<TokenTree>,
}
fn parse_matrix_mul_info(attr: proc_macro::TokenStream, structure: &DeriveInput, fields: &Vec<TokenTree>) -> MatrixMulInfo {
    let attributes: Vec<syn::MetaList> = {
        let parser = Punctuated::<syn::MetaList, Token![,]>::parse_terminated;
        parser.parse(attr).unwrap()
                    .into_iter().collect()
    };
    let mut info = MatrixMulInfo {
        ty: syn::Ident::new(structure.ident.to_string().as_str(), Span::call_site()),
        output: syn::Ident::new(structure.ident.to_string().as_str(), Span::call_site()),
        columns: fields.clone(),
        rows: fields.clone(),
    };
    for attribute in attributes {
        match attribute.path.get_ident().expect("Expected ident").to_string().as_str() {
            "ty" => {
                let tokens = attribute.tokens.clone();
                info.ty = syn::Ident::new(tokens.to_string().as_str(), Span::call_site());
            }
            "output" => {
                let tokens = attribute.tokens.clone();
                info.output = syn::Ident::new(tokens.to_string().as_str(), Span::call_site());
            }
            "columns" => {
                let attr: proc_macro::TokenStream = attribute.tokens.clone().into();
                let parser = Punctuated::<TokenTree, Token![,]>::parse_terminated;

                let fields: Vec<TokenTree> = parser.parse(attr).unwrap()
                    .into_iter().collect();
                
                info.columns = fields;
            }
            "self_rows" => {
                let attr: proc_macro::TokenStream = attribute.tokens.clone().into();
                let parser = Punctuated::<TokenTree, Token![,]>::parse_terminated;

                let fields: Vec<TokenTree> = parser.parse(attr).unwrap()
                    .into_iter().collect();
                
                info.rows = fields;
            }
            _ => {
                panic!("Attribute not recognized")
            }
        }
    }
    info
}
/// Attributes:
/// mul(
///     type()
///     columns()
///     self_rows()
///     output()
/// )
pub fn matrix_multiplication_inner(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item2 = proc_macro2::TokenStream::from(item.clone());

    let structure = parse_macro_input!(item as DeriveInput);
    let (fields, is_tuple, is_first_element_array) = get_fields(&structure);
    
    let mul_info = parse_matrix_mul_info(attr, &structure, &fields);
    
    let name = structure.ident.clone();
    let parameters = get_parameters(&structure);
    let generics = structure.generics.clone();
    
    let params = generics.params.clone();
    
    let mul_function = to_mul_function(&fields, &name, &mul_info);
    println!("{}", mul_function.clone());
    item2.append_all(mul_function);
    item2.into()
}