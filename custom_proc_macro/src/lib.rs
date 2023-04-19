use proc_macro::{TokenStream};
use std::ops::Deref;
use syn::{ExprRepeat, parse_macro_input, Lit, Expr};
use syn::__private::quote::quote;
use syn::parse_macro_input::parse;

/// 生成数组的宏 主要用于没有实现copy语义的结构体 在无法使用[T; 32] 这种方式生成数组的情况下
///
/// # Arguments
///
/// * `input`: TokenStream(ExprRepeat) 以分号(;)为分割符, 第一个参数为表达式, 第二个参数为数量. 例: T::default(); 16
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// struct Test;
/// let arr: [Test; 16] = arr!([Test; 16]);
/// ```
/// # Expansion
/// ```
/// [Test; 16];
/// [0; 16]
/// ```
#[proc_macro]
pub fn arr(input: TokenStream) -> TokenStream {
    let repeat_expr: ExprRepeat = parse(input)
        .expect("like arr!([Test; 16])");

    let mut len = 0;
    // 获取表达式中的长度信息并转为usize
    if let Expr::Lit(expr_lit) = repeat_expr.len.deref() {
        if let Lit::Int(int_lit) = &expr_lit.lit {
            len = int_lit.base10_parse::<usize>().expect("Failed to parse integer literal");
        }
    }
    // 解析并拼接成数组
    let _expr = repeat_expr.expr;
    // 1.生成数组中的一个元素
    let _one = quote! { #_expr, };
    let mut _all = quote!();
    for _ in 0..len {
        // 2.将数组中的每个元素向数组中追加
        _all = quote! { #_all #_one };
    }
    // 3.加上中括号
    let arr = quote! { [ #_all ] };
    return arr.into();
}

#[test]
fn test_arr() {
    let int_arr = arr!([u32; 12]);
}