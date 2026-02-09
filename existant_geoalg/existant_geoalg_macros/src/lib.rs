use crate::matrix::matrix_multiplication_inner;

mod matrix;
/// Attributes:
/// mul(
///     type()
///     columns()
///     self_rows()
///     output()
/// )
/// This macro implements a matrix multiplication between any nxm * mxn
#[proc_macro_attribute]
pub fn matrix_multiplication(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    matrix_multiplication_inner(attr, item)
}
