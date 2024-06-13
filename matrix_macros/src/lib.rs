use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Error, ExprArray, Result,
};

struct MatrixInput {
    arrays: Punctuated<ExprArray, Comma>,
}

impl Parse for MatrixInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let arrays = Punctuated::parse_terminated(input)?;
        Ok(MatrixInput { arrays })
    }
}

#[proc_macro]
pub fn matrix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MatrixInput);

    match process_matrix(input) {
        Ok(matrix) => matrix.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn process_matrix(input: MatrixInput) -> Result<TokenStream2> {
    let mut buf = Vec::new();
    let mut cols = None;

    for array in input.arrays {
        let row_len = array.elems.len();
        if let Some(expected_cols) = cols {
            if row_len != expected_cols {
                return Err(Error::new_spanned(
                    array,
                    "All rows must have the same number of columns",
                ));
            }
        } else {
            cols = Some(row_len);
        }
        buf.extend(array.elems);
    }

    let cols = cols.unwrap_or(0);

    Ok(quote! {
        matrix::Matrix::from_vec(#cols, vec![#(#buf),*])
    })
}
