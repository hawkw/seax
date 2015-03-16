pub use self::SVMCell::*;
pub use self::Atom::*;

use super::slist::List;

use std::fmt;
use std::ops;

#[derive(PartialEq,Clone,Debug)]
pub enum SVMCell {
    AtomCell(Atom),
    ListCell(Box<List<SVMCell>>),
    InstCell(Inst)
}

impl fmt::Display for SVMCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self)
    }
}

/// SVM atom types.
///
/// A VM atom can be either an unsigned int, signed int, float, or char.
#[derive(PartialEq,PartialOrd,Copy,Clone,Debug)]
pub enum Atom {
    /// Unsigned integer atom (machine size)
    UInt(usize),
    /// Signed integer atom (machine size)
    SInt(isize),
    /// Floating point number atom (64-bits)
    Float(f64),
    /// UTF-8 character atom
    Char(char)
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Atom::UInt(value) => write!(f, "{}", value),
            &Atom::SInt(value) => write!(f, "{}", value),
            &Atom::Float(value) => write!(f, "{}", value),
            &Atom::Char(value) => write!(f, "'{}'", value),
        }
    }
}

impl ops::Add for Atom {
    type Output = Atom;

    fn add(self, other: Atom) -> Atom {
        match (self, other) {
            // same type:  no coercion
            (SInt(a), SInt(b))      => SInt(a + b),
            (UInt(a), UInt(b))      => UInt(a + b),
            (Float(a), Float(b))    => Float(a + b),
            (Char(a), Char(b))      => Char((a as u8 + b as u8) as char),
            // float + int: coerce to float
            (Float(a), SInt(b))     => Float(a + b as f64),
            (Float(a), UInt(b))     => Float(a + b as f64),
            (SInt(a), Float(b))     => Float(a as f64 + b),
            (UInt(a), Float(b))     => Float(a as f64 + b),
            // uint + sint: coerce to sint
            (UInt(a), SInt(b))      => SInt(a as isize + b),
            (SInt(a), UInt(b))      => SInt(a + b as isize),
            // char + any: coerce to char
            // because of the supported operations on Rusizet chars,
            // everything has to be cast to u8 (byte) to allow
            // arithmetic ops and then cast back to char.
            (Char(a), UInt(b))      => Char((a as u8 + b as u8) as char),
            (Char(a), SInt(b))      => Char((a as u8 + b as u8) as char),
            (Char(a), Float(b))     => Char((a as u8 + b as u8) as char),
            (UInt(a), Char(b))      => Char((a as u8 + b as u8) as char),
            (SInt(a), Char(b))      => Char((a as u8 + b as u8) as char),
            (Float(a), Char(b))     => Char((a as u8 + b as u8) as char)
        }
    }

}

impl ops::Sub for Atom {
    type Output = Atom;

    fn sub(self, other: Atom) -> Atom {
        match (self, other) {
            // same type:  no coercion
            (SInt(a), SInt(b))      => SInt(a - b),
            (UInt(a), UInt(b))      => UInt(a - b),
            (Float(a), Float(b))    => Float(a - b),
            (Char(a), Char(b))      => Char((a as u8 - b as u8) as char),
            // float + int: coerce to float
            (Float(a), SInt(b))     => Float(a - b as f64),
            (Float(a), UInt(b))     => Float(a - b as f64),
            (SInt(a), Float(b))     => Float(a as f64 - b),
            (UInt(a), Float(b))     => Float(a as f64 - b),
            // uint + sint: coerce to sint
            (UInt(a), SInt(b))      => SInt(a as isize - b),
            (SInt(a), UInt(b))      => SInt(a - b as isize),
            // char + any: coerce to char
            (Char(a), UInt(b))      => Char((a as u8 - b as u8) as char),
            (Char(a), SInt(b))      => Char((a as u8 - b as u8) as char),
            (Char(a), Float(b))     => Char((a as u8 - b as u8) as char),
            (UInt(a), Char(b))      => Char((a as u8 - b as u8) as char),
            (SInt(a), Char(b))      => Char((a as u8 - b as u8) as char),
            (Float(a), Char(b))     => Char((a as u8 - b as u8) as char)
        }
    }

}

impl ops::Div for Atom {
    type Output = Atom;

    fn div(self, other: Atom) -> Atom {
        match (self, other) {
            // same type:  no coercion
            (SInt(a), SInt(b))      => SInt(a / b),
            (UInt(a), UInt(b))      => UInt(a / b),
            (Float(a), Float(b))    => Float(a / b),
            (Char(a), Char(b))      => Char((a as u8 / b as u8) as char),
            // float + int: coerce to float
            (Float(a), SInt(b))     => Float(a / b as f64),
            (Float(a), UInt(b))     => Float(a / b as f64),
            (SInt(a), Float(b))     => Float(a as f64 / b),
            (UInt(a), Float(b))     => Float(a as f64 / b),
            // uint + sint: coerce to sint
            (UInt(a), SInt(b))      => SInt(a as isize / b),
            (SInt(a), UInt(b))      => SInt(a / b as isize),
            // char + any: coerce to char
            (Char(a), UInt(b))      => Char((a as u8 / b as u8) as char),
            (Char(a), SInt(b))      => Char((a as u8 / b as u8) as char),
            (Char(a), Float(b))     => Char((a as u8 / b as u8) as char),
            (UInt(a), Char(b))      => Char((a as u8 / b as u8) as char),
            (SInt(a), Char(b))      => Char((a as u8 / b as u8) as char),
            (Float(a), Char(b))     => Char((a as u8 / b as u8) as char)
        }
    }

}

impl ops::Mul for Atom {
    type Output = Atom;

    fn mul(self, other: Atom) -> Atom {
        match (self, other) {
            // same type:  no coercion
            (SInt(a), SInt(b))      => SInt(a * b),
            (UInt(a), UInt(b))      => UInt(a * b),
            (Float(a), Float(b))    => Float(a * b),
            (Char(a), Char(b))      => Char((a as u8 * b as u8) as char),
            // float + int: coerce to float
            (Float(a), SInt(b))     => Float(a * b as f64),
            (Float(a), UInt(b))     => Float(a * b as f64),
            (SInt(a), Float(b))     => Float(a as f64* b),
            (UInt(a), Float(b))     => Float(a as f64* b),
            // uint + sint: coerce to sint
            (UInt(a), SInt(b))      => SInt(a as isize * b),
            (SInt(a), UInt(b))      => SInt(a * b as isize),
            // char + any: coerce to char
            (Char(a), UInt(b))      => Char((a as u8 * b as u8) as char),
            (Char(a), SInt(b))      => Char((a as u8 * b as u8) as char),
            (Char(a), Float(b))     => Char((a as u8 * b as u8) as char),
            (UInt(a), Char(b))      => Char((a as u8 * b as u8) as char),
            (SInt(a), Char(b))      => Char((a as u8 * b as u8) as char),
            (Float(a), Char(b))     => Char((a as u8 * b as u8) as char)
        }
    }

}

impl ops::Rem for Atom {
    type Output = Atom;

    fn rem(self, other: Atom) -> Atom {
        match (self, other) {
            // same type:  no coercion
            (SInt(a), SInt(b))      => SInt(a % b),
            (UInt(a), UInt(b))      => UInt(a % b),
            (Float(a), Float(b))    => Float(a % b),
            (Char(a), Char(b))      => Char((a as u8 % b as u8) as char),
            // float + int: coerce to float
            (Float(a), SInt(b))     => Float(a % b as f64),
            (Float(a), UInt(b))     => Float(a % b as f64),
            (SInt(a), Float(b))     => Float(a as f64 % b),
            (UInt(a), Float(b))     => Float(a as f64 % b),
            // uint + sint: coerce to sint
            (UInt(a), SInt(b))      => SInt(a as isize % b),
            (SInt(a), UInt(b))      => SInt(a % b as isize),
            // char + any: coerce to char
            (Char(a), UInt(b))      => Char((a as u8 % b as u8) as char),
            (Char(a), SInt(b))      => Char((a as u8 % b as u8) as char),
            (Char(a), Float(b))     => Char((a as u8 % b as u8) as char),
            (UInt(a), Char(b))      => Char((a as u8 % b as u8) as char),
            (SInt(a), Char(b))      => Char((a as u8 % b as u8) as char),
            (Float(a), Char(b))     => Char((a as u8 % b as u8) as char)
        }
    }

}

/// SVM instruction types
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Inst {
    /// `nil`
    ///
    /// Pusizehes an empty list (nil) onto the stack
    NIL,
    /// `ldc`: `L`oa`d` `C`onstant. Loads a constant (atom)
    LDC,
    /// `ld`: `L`oa`d`. Pusizehes a variable onto the stack.
    ///
    /// The variable is indicated by the argument, a pair.
    /// The pair's `car` specifies the level, the `cdr` the position.
    /// So `(1 . 3)` gives the current function's (level 1) third
    /// parameter.
    LD,
    /// `ldf`: `L`oa`d` `F`unction.
    ///
    ///  Takes one list argument representing a function and constructs
    ///  a closure (a pair containing the function and the current
    ///  environment) and pusizehes that onto the stack.
    LDF,
    /// `join`
    ///
    /// Pops a list reference from the dump and makes thisize the new value
    /// of `C`. This instruction occurs at the end of both alternatives of
    ///  a `sel`.
    JOIN,
    /// `ap`: `Ap`ply.
    ///
    /// Pops a closure and a list of parameter values from the stack.
    /// The closure is applied to the parameters by installing its
    /// environment as the current one, pushing the parameter list
    /// in front of that, clearing the stack, and setting `C` to the
    /// closure's function pointer. The previous values of `S`, `E`,
    ///  and the next value of `C` are saved on the dump.
    AP,
    /// `ret`: `Ret`urn.
    ///
    /// Pops one return value from the stack, restores
    /// `S`, `E`, and `C` from the dump, and pusizehes
    /// the return value onto the now-current stack.
    RET,
    /// `dum`: `Dum`my.
    ///
    /// Pops a dummy environment (an empty list) onto the `E` stack.
    DUM,
    /// `rap`: `R`ecursive `Ap`ply.
    /// Works like `ap`, only that it replaces an occurrence of a
    /// dummy environment with the current one, thusize making recursive
    ///  functions possible.
    RAP,
    /// `sel`: `Sel`ect branch
    ///
    /// Expects two list arguments on the control stack, and pops a value
    /// from the stack. The first list is executed if the popped value
    /// was non-nil, the second list otherwise. Before one of these list
    /// pointers is made the new `C`, a pointer to the instruction
    /// following `sel` is saved on the dump.
    SEL,
    /// `add`
    ///
    /// Pops two numbers off of the stack and adds them, pusizehing the
    /// result onto the stack. Thisize will up-convert integers to floating
    /// point if necessary.
    ///
    /// TODO: figure out what happens when you try to add things that aren't
    /// numbers (maybe the compiler won't let thisize happen?).
    ADD,
    /// `sub`: `Sub`tract
    ///
    /// Pops two numbers off of the stack and subtracts the first from the
    /// second, pusizehing the result onto the stack. This will up-convert
    /// integers to floating point if necessary.
    ///
    /// TODO: figure out what happens when you try to subtract things that
    /// aren't numbers (maybe the compiler won't let thisize happen?).
    SUB,
    /// `mul`: `Mul`tiply
    ///
    /// Pops two numbers off of the stack and multiplies them, pusizehing the
    /// result onto the stack. This will up-convert integers to floating
    /// point if necessary.
    ///
    /// TODO: figure out what happens when you try to multiply things that
    /// aren't numbers (maybe the compiler won't let thisize happen?).
    MUL,
    /// `div`: `Div`ide
    ///
    /// Pops two numbers off of the stack and divides the first by the second,
    /// pushing the result onto the stack. This performs integer divisizeion.
    ///
    /// TODO: figure out what happens when you try to divide things that
    /// aren't numbers (maybe the compiler won't let thisize happen?).
    DIV,
    /// `fdiv`: `F`loating-point `div`ide
    ///
    /// Pops two numbers off of the stack and divides the first by the second,
    /// pusizehing the result onto the stack. This performs float divisizeion.
    ///
    /// TODO: figure out what happens when you try to divide things that
    /// aren't numbers (maybe the compiler won't let this happen?).
    ///
    /// TODO: Not sure if there should be separate float and int divide words
    /// I guess the compiler can figure this out
    FDIV,
    /// `mod`: `Mod`ulo
    ///
    /// Pops two numbers off of the stack and divides the first by the second,
    /// pushing the remainder onto the stack.
    ///
    /// TODO: figure out what happens when you try to modulo things that
    /// aren't numbers (maybe the compiler won't let this happen?).
    MOD,
    /// `eq`: `Eq`uality of atoms
    EQ,
    /// `gt`: `G`reater `t`han
    ///
    /// Pops two numbers on the stack and puts a 'true' on the stack
    /// if the first atom isize greater than the other atom, false otherwisizee.
    GT,
    /// `gte`: `G`reater `t`han or `e`qual
    GTE,
    /// `lt`: `L`ess `t`han
    LT,
    /// `lte`: `L`ess `t`han or `e`qual
    LTE,
    /// `atom`: test if `atom`
    ///
    /// Pops an item from the stack and returns true if it's an atom, false
    /// otherwise.
    ATOM,
    /// `car`: `C`ontents of `A`ddress `R`egister
    ///
    /// Pops a list from the stack and returns the list's `car` (head)
    CAR,
    /// `cdr`: `C`ontents of `D`ecrement `R`egister
    ///
    /// Pops a list from the stack and returns the list's `cdr` (tail)
    CDR,
    /// `cons`: `Cons`truct
    ///
    /// Pops an item and a list from the stack and returns the list, with
    /// the item prepended.
    CONS,
    /// `null`: test if `null`
    ///
    ///  Pops an item from the stack and returns true if it is `nil`, false
    ///  otherwise.
    NULL,
    /// `stop`: `stop` execution
    ///
    /// Terminates program execution. The `eval_program()` function will return
    /// the last state of the VM.
    STOP
    // TODO: add some hardcoded I/O instructions here so that you can
    //  do I/O without farming everything out to `stdio`
}

#[cfg(test)]
mod tests {
    use super::Atom;
    use super::Atom::*;
    #[test]
    fn test_atom_show () {
        let mut a: Atom;

        a = Char('a');
        assert_eq!(format!("{}", a), "'a'");

        a = UInt(1usize);
        assert_eq!(format!("{}", a), "1");

        a = SInt(42isize);
        assert_eq!(format!("{}", a), "42");

        a = SInt(-1isize);
        assert_eq!(format!("{}", a), "-1");

        a = Float(5.55f64);
        assert_eq!(format!("{}", a), "5.55");

        a = Float(1f64);
        assert_eq!(format!("{}", a), "1");

    }
}