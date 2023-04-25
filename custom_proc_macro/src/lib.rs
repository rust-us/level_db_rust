use proc_macro::{TokenStream};
use std::ops::Deref;
use syn::{ExprRepeat, Lit, Expr};
use syn::__private::quote::quote;
use syn::parse;

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
        .expect("Like arr!([Test::new(); 16])");

    let mut len = 0;
    // 获取表达式中的长度信息并转为usize
    if let Expr::Lit(expr_lit) = repeat_expr.len.deref() {
        if let Lit::Int(int_lit) = &expr_lit.lit {
            len = int_lit.base10_parse::<usize>().expect("Failed to parse integer literal");
        }
    }
    // 解析并拼接成数组
    let _expr = repeat_expr.expr;
    // 1.生成数组的集合
    let mut _all = quote!();
    for _i in 0..len {
        // 2.将每个元素向数组中追加
        if let Expr::Path(path) = _expr.as_ref() {
            // 如果是element宏的情况会调用element宏并传入index
            let _mac_name = &path;
            _all = quote! { #_all #_mac_name!(#_i, capacity, default_length), };
        } else {
            _all = quote! { #_all #_expr, };
        }
    }
    // 3.加上中括号
    let arr = quote! { [ #_all ] };
    return arr.into();
}

/// 生成调用NonNull::new_unchecked()的方法, 会自动包裹unsafe{}代码块
#[proc_macro]
pub fn non_null_new_uncheck(input: TokenStream) -> TokenStream {
    let ptr_expr: Expr = parse(input.into())
        .expect("Like non_null_new_uncheck!(ptr), ptr must a variable with a raw point");
    let output = quote! { unsafe { std::ptr::NonNull::new_unchecked(#ptr_expr) } };
    output.into()
}