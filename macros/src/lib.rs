extern crate proc_macro;

#[proc_macro]
pub fn remaining_interrupt_handlers(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut out_string = String::new();
    for num in 32..256 {
        out_string.push_str(&["extern \"x86-interrupt\" fn isr","(stack: InterruptStackFrame) {handle_interrupt!(no_err "," \"Interrupt #","\", stack);}"].join(num.to_string().as_str()));
    }
    out_string.parse().unwrap()
}

#[proc_macro]
pub fn set_remaining_interrupt_handlers(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut out_string = String::new();
    for num in 32..256 {
        out_string.push_str(&["IDT[","].set_handler_fn(isr",");"].join(num.to_string().as_str()));
    }
    out_string.parse().unwrap()
}