mod analysis;
mod codegen;

use analysis::VariableAnalyses;
use proc_macro::TokenStream;
use codegen::codegen_statement;

/// Derive macro for the module.
#[proc_macro_attribute]
pub fn cube(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let func: syn::ItemFn = syn::parse(tokens).unwrap();
    let mut variable_analyses = VariableAnalyses::create(&func);

    codegen_cube(&func, &mut variable_analyses)
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct VariableKey {
    name: String,
}

impl From<&syn::Ident> for VariableKey {
    fn from(value: &syn::Ident) -> Self {
        VariableKey {
            name: value.to_string(),
        }
    }
}

/// Generate the expanded version of a function marked with the cube macro
fn codegen_cube(func: &syn::ItemFn, variable_analyses: &mut VariableAnalyses) -> TokenStream {
    let signature = expand_sig(&func.sig);
    let mut body = quote::quote! {};

    // panic!("WG");
    for statement in func.block.stmts.iter() {
        let tokens = codegen_statement(statement, 0, variable_analyses);
        body.extend(tokens);
    }

    let code = quote::quote! {
        #func

        #[allow(unused_mut)]
        #signature {
            #body
        }
    }
    .into();
    // panic!("{code}");

    code
}

fn expand_sig(sig: &syn::Signature) -> proc_macro2::TokenStream {
    let mut inputs = quote::quote!();

    for input in &sig.inputs {
        match input {
            syn::FnArg::Typed(pat) => {
                let ty = &pat.ty;
                let ident = pat.pat.clone();

                inputs.extend(quote::quote! {
                    #ident: <#ty as burn_cube::RuntimeType>::ExpandType,
                });
            }
            _ => todo!(),
        }
    }

    let mut output = quote::quote!();

    match &sig.output {
        syn::ReturnType::Default => output.extend(quote::quote! { ()}),
        syn::ReturnType::Type(_, ty) => {
            output.extend(quote::quote! {
                <#ty as burn_cube::RuntimeType>::ExpandType
            });
        }
    }

    let ident = &sig.ident;
    let ident = syn::Ident::new(format!("{ident}_expand").as_str(), ident.span());

    quote::quote! {
        pub fn #ident(context: &mut burn_cube::CubeContext, #inputs) -> #output
    }
    .into()
}