///
/// Macro to mimick Python's slicing.
///
macro_rules! slice {
    ( $e:expr , [ ; $end_expr:expr ] ) => {
        $e [..$end_expr]
    };
    ( $e:expr , [ $start_expr:expr ; $end_expr:expr] ) => {
        $e [$start_expr..$end_expr]
    };
    ( $e:expr , [ $start_expr:expr ; ] ) => {
        $e [$start_expr..]
    };
    ( $e:expr , [ $start_expr:expr ; $end_expr:expr ; $steps_expr:expr] ) => {
        std::unimplemented();
    };
    ( $e:expr , [ ; $end_expr:expr ; $steps_expr:expr ] ) => {
        std::unimplemented();
    };
    ( $e:expr , [ ; ] ) => {
        std::unimplemented();
    };
}

///
/// Trait for cohercing values types to 'bool', like it is done in
/// the Python's `if`.
///
pub trait Boolable {
    fn get_bool_value(self) -> bool;
}

/*

Wanted to do this (but it conflicted with my other impls:

use num;

impl<T> Boolable for T where T : num::Zero {
    fn get_bool_value(&self) -> bool {
        !self.is_zero()
    }
}
*/

impl     Boolable for     u32 { fn get_bool_value(self) -> bool { self != 0 }}
impl<'a> Boolable for &'a u32 { fn get_bool_value(self) -> bool { *self != 0 }}
impl     Boolable for     u16 { fn get_bool_value(self) -> bool { self != 0 }}
impl<'a> Boolable for &'a u16 { fn get_bool_value(self) -> bool { *self != 0 }}
impl     Boolable for      u8 { fn get_bool_value(self) -> bool { self != 0 }}
impl<'a> Boolable for &'a  u8 { fn get_bool_value(self) -> bool { *self != 0 }}
impl     Boolable for     i32 { fn get_bool_value(self) -> bool { self != 0 }}
impl<'a> Boolable for &'a i32 { fn get_bool_value(self) -> bool { *self != 0 }}
impl     Boolable for     i16 { fn get_bool_value(self) -> bool { self != 0 }}
impl<'a> Boolable for &'a i16 { fn get_bool_value(self) -> bool { *self != 0 }}
impl     Boolable for      i8 { fn get_bool_value(self) -> bool { self != 0 }}
impl<'a> Boolable for &'a  i8 { fn get_bool_value(self) -> bool { *self != 0 }}
impl<'a, T>
         Boolable for &'a [T] { fn get_bool_value(self) -> bool { self.len() != 0 }}
impl<'a>
         Boolable for &'a str { fn get_bool_value(self) -> bool { self.len() != 0 }}
impl     Boolable for bool    { fn get_bool_value(self) -> bool { self }}
impl     Boolable for ()      { fn get_bool_value(self) -> bool { false }}
impl<T1> Boolable for (T1, )  { fn get_bool_value(self) -> bool { true  }}
impl<T1, T2> Boolable for (T1, T2)
                              { fn get_bool_value(self) -> bool { true  }}
impl<T1, T2, T3> Boolable for (T1, T2, T3)
                              { fn get_bool_value(self) -> bool { true  }}

/// Similar to the Python [all](https://docs.python.org/2/library/functions.html#all) builtin.
pub fn all<T, Z>(x : T) -> bool where Z : Boolable, T : Iterator<Item = Z> {
    for v in x {
        if !v.get_bool_value() {
            return false
        }
    }
    true
}

/// Similar to the Python [any](https://docs.python.org/2/library/functions.html#any) builtin.
pub fn any<T, Z>(x : T) -> bool where Z : Boolable, T : Iterator<Item = Z> {
    for v in x {
        if v.get_bool_value() {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice() {
        let v : Vec<u32> = vec![1,2,3,4,5,6];

        assert_eq!(&v[2..4], [3, 4]);
        assert_eq!(&slice!(v, [;4]), [1, 2, 3, 4]);
        assert_eq!(&slice!(v, [4;]), [5, 6]);
    }

    #[test]
    fn bool() {
        use super::{Boolable};

        assert_eq!(3.get_bool_value(), true);
        assert_eq!(0.get_bool_value(), false);
        let x : [u32; 0] = [];
        assert_eq!(x.get_bool_value(), false);
        let y : [u32; 1] = [1];
        assert_eq!(y.get_bool_value(), true);
        assert_eq!(true.get_bool_value(), true);
        assert_eq!(false.get_bool_value(), false);
        assert_eq!(" xx".get_bool_value(), true);
        assert_eq!("".get_bool_value(), false);
    }

    #[test]
    fn all_and_any() {
        let empty : [u32;0] = [];
        assert_eq!(super::all(empty.into_iter()), true);
        assert_eq!(super::all([2, 3, 4].into_iter()), true);
        assert_eq!(super::all([2, 0, 4].into_iter()), false);
        assert_eq!(super::any(empty.into_iter()), false);
        assert_eq!(super::any([2, 3, 4].into_iter()), true);
        assert_eq!(super::any([2, 0, 4].into_iter()), true);
        assert_eq!(super::any([0, 0, 0].into_iter()), false);
    }
}
