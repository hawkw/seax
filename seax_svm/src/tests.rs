use ::slist::Stack;
use ::slist::List::{Cons,Nil};
use super::State;
use super::cell::Atom::*;
use super::cell::SVMCell::*;
use super::Inst::*;
use std::io;

#[test]
#[should_panic(expected="[fatal]: expected an instruction on control stack")]
fn test_empty_eval_fail() {
    State::new().eval(&mut io::stdin(), &mut io::stdout(),false);
}

#[test]
#[should_panic(expected="List index 0 out of range")]
fn test_ld_empty_env_fail() {
    State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(1)), AtomCell(SInt(0))))),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][LD]: expected list in $e, found 'w'")]
fn test_ld_unexpected_env_fail() {
    State {
        stack:      Stack::empty(),
        env:        list!(AtomCell(Char('w'))),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(1)), AtomCell(SInt(1))))),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="fatal][LD]: expected pair, found (0 . nil)\n[fatal] new control: nil")]
fn test_ld_arg_too_short_fail() {
    State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(0))))),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}
#[test]
#[should_panic(expected="[fatal][LD]: expected pair, found (0 . (1 . (1 . nil)))\n[fatal] new control: nil")]
fn test_ld_arg_too_long_fail() {
    State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(0)), AtomCell(SInt(1)), AtomCell(SInt(1))))),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][ADD]: expected first operand, found Some(((1 . nil), nil))")]
fn test_add_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(ADD)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}


#[test]
#[should_panic(expected="[fatal][SUB]: expected first operand, found Some(((1 . nil), nil))")]
fn test_sub_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(SUB)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][DIV]: expected first operand, found Some(((1 . nil), nil))")]
fn test_div_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(DIV)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][FDIV]: Expected first operand to be atom, found list or instruction")]
fn test_fdiv_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(FDIV)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][MUL]: expected first operand, found Some(((1 . nil), nil))")]
fn test_mul_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(MUL)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][ADD]: expected second operand, found Some((nil, nil))")]
fn test_add_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), ListCell(box Nil)),
        env:        Stack::empty(),
        control:    list!(InstCell(ADD)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}
#[test]
#[should_panic(expected="[fatal][SUB]: expected second operand, found Some((nil, nil))")]
fn test_sub_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), ListCell(box Nil)),
        env:        Stack::empty(),
        control:    list!(InstCell(SUB)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][DIV]: expected second operand, found Some((nil, nil))")]
fn test_div_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), ListCell(box Nil)),
        env:        Stack::empty(),
        control:    list!(InstCell(DIV)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][FDIV]: TypeError: expected compatible operands, found (FDIV 1 nil)")]
fn test_fdiv_type_error () {
   State {
        stack:      list!(AtomCell(SInt(1)), ListCell(box Nil)),
        env:        Stack::empty(),
        control:    list!(InstCell(FDIV)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
#[should_panic(expected="[fatal][MUL]: expected second operand, found Some((nil, nil))")]
fn test_mul_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), ListCell(box Nil)),
        env:        Stack::empty(),
        control:    list!(InstCell(MUL)),
        dump:       Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), false);
}

#[test]
fn test_empty_state() {
    let state = State::new();
    assert_eq!(state.stack.length(), 0);
    assert_eq!(state.env.length(), 0);
    assert_eq!(state.control.length(), 0);
    assert_eq!(state.dump.length(), 0);
}

#[test]
fn test_eval_nil () {
    let mut state = State {
        stack: Stack::empty(),
        env: Stack::empty(),
        control: list!(InstCell(NIL),AtomCell(SInt(1))),
        dump: Stack::empty()
    };
    assert_eq!(state.stack.peek(), None);
    state = state.eval(&mut io::stdin(), &mut io::stdout(),true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));
}

#[test]
fn test_eval_ldc () {
    let mut state = State::new();
    assert_eq!(state.stack.peek(), None);
    state = State {
        stack: state.stack,
        env: state.env,
        control: list!(InstCell(LDC),AtomCell(SInt(1))),
        dump: state.dump
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(1))));

    state = State {
        stack: state.stack,
        env: state.env,
        control: list!(InstCell(LDC),AtomCell(Char('a'))),
        dump: state.dump
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Char('a'))));

    state = State {
        stack: state.stack,
        env: state.env,
        control: list!(InstCell(LDC),AtomCell(Float(1.0f64))),
        dump: state.dump
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.0f64))));
}

#[test]
fn test_eval_ld () {
    let state = State {
        stack: Stack::empty(),
        env: list!(ListCell(box list!(AtomCell(SInt(155)),AtomCell(UInt(388))))),
        control: list!(
            InstCell(LD),
            ListCell(
                box list!(
                    AtomCell(SInt(1)),AtomCell(SInt(2))
                    )
                )
            ),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(388))));
}

#[test]
fn test_eval_ldf () {
        let state = State {
        stack: Stack::empty(),
        env: list!(
            ListCell(
                box list!(
                    AtomCell(SInt(155)),
                    AtomCell(UInt(388))
                )
            ),
            ListCell(
                box list!(
                    AtomCell(Float(6.66)),
                    AtomCell(SInt(666))
                    )
                )
            ),
        control: list!(InstCell(LDF), ListCell(box list!(AtomCell(SInt(133))))),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(
        state.stack.peek(),
        Some(&ListCell(
            box list!(
                ListCell(
                    box list!(
                        AtomCell(SInt(133))
                    )),
                ListCell(
                    box list!(
                        AtomCell(SInt(155)),
                        AtomCell(UInt(388))
                    ),
                )
            )
        )
    )
);
}

#[test]
fn test_eval_join() {
    let state = State {
        stack: Stack::empty(),
        env: Stack::empty(),
        control: list!(InstCell(JOIN)),
        dump: list!(ListCell(box list!(
                AtomCell(SInt(1)),
                AtomCell(SInt(2))
            )))
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.dump.peek(), None);
    assert_eq!(state.control[0usize], AtomCell(SInt(1)));
    assert_eq!(state.control[1usize], AtomCell(SInt(2)));
}

#[test]
fn test_eval_add () {
    // ---- Unsigned int addition ----
    let mut state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(2))));

    // ---- Signed int addition ----
    state = State {
        stack: list!(AtomCell(SInt(-1)), AtomCell(SInt(-1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-2))));

    // ---- Float-float addition ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(Float(1.5))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

    // ---- Float-int type lifting addition ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(2.5))));
    state = State {
        stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(4.5))));
}

#[test]
fn test_eval_sub () {
    // ---- Unsigned int subtraction ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(0))));

    // ---- Signed int subtraction----
    state = State {
        stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-6))));

    // ---- Float-float subtraction ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(-0.5))));

    // ---- Float-int type lifting subtraction ----
    state = State {
        stack: list!(AtomCell(Float(2.5)), AtomCell(SInt(-2))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(4.5))));

    state = State {
        stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
}

#[test]
fn test_eval_mul () {
    // ---- Unsigned int multiplication ----
    let mut state = State {
        stack: list!(AtomCell(UInt(2)), AtomCell(UInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(6))));

    // ---- Signed int multiplication----
    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(-3))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(6))));

    // ---- Float-float multiplication ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

    // ---- Float-int type lifting multiplication ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

    state = State {
        stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(7.0))));
}

#[test]
fn test_eval_div () {
    // ---- Unsigned int divison ----
    let mut state = State {
        stack: list!(AtomCell(UInt(6)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(3))));

    // ---- Signed int divison ----
    state = State {
        stack: list!(AtomCell(SInt(-6)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-3))));

    // ---- Float-float divison ----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

    // ---- Float-int type lifting divison ----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
}

#[test]
fn test_eval_fdiv () {
    // ---- Unsigned int divison ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(FDIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

    // ---- Signed int divison ----
    state = State {
        stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(FDIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(-1.5))));

    // ---- Float-float divison ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(FDIV)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
}

#[test]
fn test_eval_mod () {
    // ---- Unsigned int modulus ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(3%2))));

    // ---- Signed int modulus ----
    state = State {
        stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-3%2))));

    // ---- Float-float modulus---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));

    // ---- Float-int type lifting modulus----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));

    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));
}

#[test]
fn test_eval_eq () {
    // ---- Unsigned int equality ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    // ---- Signed int equality ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));


    // ---- Float equality ----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(3.0))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

}

#[test]
fn test_eval_gt () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    // ---- Mixed type greater-than ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

}

#[test]
fn test_eval_gte () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));


    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    // ---- Mixed type greater-than-equal ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));
}

#[test]
fn test_eval_lt () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));


    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

       state = State {
        stack: list!(AtomCell(Float(2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    // ---- Mixed type greater-than ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

}

#[test]
fn test_eval_lte () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );


    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    // ---- Mixed type greater-than-equal ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(
        box Nil // TODO: this expects wrong float behaviour, fix
        ))
    );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );
}

#[test]
fn test_eval_ret() {
    let state = State {
        stack: list!(AtomCell(SInt(100)), AtomCell(SInt(320))),
        env: Stack::empty(),
        control: list!(InstCell(RET)),
        dump: list!(
            ListCell(box list!(AtomCell(Char('S')), AtomCell(Char('L')))),
            ListCell(box list!(
                ListCell(box list!(AtomCell(Char('E')), AtomCell(Char('L')))),
                ListCell(box list!(AtomCell(Char('E')), AtomCell(Char('D'))))
                )),
            ListCell(box list!(AtomCell(Char('C')), AtomCell(Char('L'))))
            )
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    // stack should have return arg + first elem on dump
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(100)))); // test these using peek for now since indexing is borked
    assert_eq!(state.stack[0usize], AtomCell(SInt(100)));
    assert_eq!(state.stack[1usize], AtomCell(Char('S')));
    assert_eq!(state.stack[2usize], AtomCell(Char('L')));
    // env should have second element from dump
    assert_eq!(state.env.peek(), Some(&ListCell(box list!(AtomCell(Char('E')), AtomCell(Char('L'))))));
    assert_eq!(state.env[0usize], ListCell(box list!(AtomCell(Char('E')), AtomCell(Char('L')))));
    assert_eq!(state.env[1usize], ListCell(box list!(AtomCell(Char('E')), AtomCell(Char('D')))));
    // control should have third element from dump
    assert_eq!(state.control.peek(), Some(&AtomCell(Char('C'))));
    assert_eq!(state.control[0usize], AtomCell(Char('C')));
    assert_eq!(state.control[1usize], AtomCell(Char('L')));
    assert_eq!(state.dump.peek(), None);
}

#[test]
fn test_eval_dum() {
    let state = State {
        stack: Stack::empty(),
        env: list!(ListCell(box list!(AtomCell(Char('a'))))),
        control: list!(InstCell(DUM)),
        dump: Stack::empty(),
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.env.peek(), Some(&ListCell(box Nil)));
}

#[test]
fn test_eval_ap() {
    let state = State {
        stack: list!(
            ListCell(box list!(
                ListCell(box list!(
                    InstCell(RET), InstCell(ADD), AtomCell(SInt(1)), InstCell(LDC), ListCell(box list!(AtomCell(UInt(0)), AtomCell(UInt(0)))),
                    InstCell(LD)
                    )),
                ListCell(box list!(
                    ListCell(box Cons(
                        AtomCell(SInt(1)), box Nil
                        ))
                    ))
                )),
            ListCell(box list!( AtomCell(Char('Q')) ))
            ),
        env: list!(ListCell(
            box Cons(AtomCell(Char('D')), box Nil)
            )),
        control: list!(InstCell(AP), InstCell(DUM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), None );
    assert_eq!(state.control, list!(InstCell(RET), InstCell(ADD), AtomCell(SInt(1)), InstCell(LDC), ListCell(box list!(AtomCell(UInt(0)), AtomCell(UInt(0)))),InstCell(LD)));
    assert_eq!(state.env, list!(
        ListCell(box list!(AtomCell(Char('Q')))),
        ListCell(box list!(AtomCell(SInt(1))))
        ));
    //assert_eq!(state.dump, list!(ListCell(box list!(InstCell(DUM))),ListCell(box list!(ListCell(box list!(AtomCell(Char('D'))))))));
}

#[test]
fn test_eval_atom() {
    // true cases
    let mut state = State {
        stack: list!(AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(0))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Char('C'))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Char('A'))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.23f64))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert!(
        state.stack.peek() != Some(&ListCell(box Nil)) &&
        state.stack.peek() != None
        );

    // false cases
    state = State {
        stack: list!(InstCell(DUM)),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));

    state = State {
        stack: list!(ListCell(box Nil)),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));
}

#[test]
fn test_eval_car() {
    let state = State {
        stack: list!(ListCell(box list!(AtomCell(Char('A')),AtomCell(Char('B'))))),
        env: Stack::empty(),
        control: list!(InstCell(CAR)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&AtomCell(Char('A'))));
}

#[test]
fn test_eval_cdr() {
    let state = State {
        stack: list!(ListCell(box list!(AtomCell(Char('A')),AtomCell(Char('B'))))),
        env: Stack::empty(),
        control: list!(InstCell(CDR)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box list!(AtomCell(Char('B'))))));
}

#[test]
fn test_eval_cons() {
    let state = State {
        stack: list!(AtomCell(Char('A')), ListCell(box list!(AtomCell(Char('B')),AtomCell(Char('C'))))),
        env: Stack::empty(),
        control: list!(InstCell(CONS)),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), Some(&ListCell(box list!(
        AtomCell(Char('A')), AtomCell(Char('B')), AtomCell(Char('C'))
        ))));
}

#[test]
fn test_eval_sel_true() {
    // true case
    let state = State {
        stack: list!(ListCell(box Nil)),
        env: Stack::empty(),
        control: list!(
            InstCell(SEL),
            ListCell(box list!(InstCell(ATOM))), // should be on stack if true
            ListCell(box list!(InstCell(NIL))), // should be on stack if false
            InstCell(JOIN) // this is just here so that we can assert that it goes on the dump
            ),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), None); // stack should be empty
    assert_eq!(state.control.peek(), Some(&InstCell(NIL)));
    assert_eq!(state.dump.peek(), Some(&ListCell(box list!(InstCell(JOIN))))); // next instruction on dump
}

#[test]
fn test_eval_sel_false() {
    // false case
    let state = State {
        stack: list!(ListCell(box list!(AtomCell(SInt(1))))),
        env: Stack::empty(),
        control: list!(
            InstCell(SEL),
            ListCell(box list!(InstCell(ATOM))), // should be on stack if true
            ListCell(box list!(InstCell(NIL))), // should be on stack if false
            InstCell(JOIN) // this is just here so that we can assert that it goes on the dump
            ),
        dump: Stack::empty()
    }.eval(&mut io::stdin(), &mut io::stdout(), true);
    assert_eq!(state.stack.peek(), None); // stack should be empty
    assert_eq!(state.control.peek(), Some(&InstCell(ATOM)));
    assert_eq!(state.dump.peek(), Some(&ListCell(box list!(InstCell(JOIN))))); // next instruction on dump
}

#[test]
fn test_eval_null() {
    // true case
    assert_eq!(
        State {
            stack: list!(AtomCell(SInt(1))),
            env: Stack::empty(),
            control: list!(InstCell(NULL)),
            dump: Stack::empty(),
        }.eval(&mut io::stdin(), &mut io::stdout(),true).stack.peek(),
        Some(&ListCell(box Nil))
        );
    // false case
    assert_eq!(
        State {
            stack: list!(ListCell(box Nil)),
            env: Stack::empty(),
            control: list!(InstCell(NULL)),
            dump: Stack::empty(),
        }.eval(&mut io::stdin(), &mut io::stdout(),true).stack.peek(),
        Some(&ListCell(box list!(AtomCell(SInt(1)))))
        );
}