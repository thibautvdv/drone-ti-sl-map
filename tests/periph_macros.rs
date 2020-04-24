use drone_core::token::Token;
use drone_ti_sl_map::ti_sl_reg_tokens;

ti_sl_reg_tokens! {
    struct Regs;
}

#[test]
#[allow(unused_variables)]
fn periph_macros8() {
    let reg = unsafe { Regs::take() };
}