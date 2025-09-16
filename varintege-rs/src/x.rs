#[cfg(not(feature = "moq"))]
#[macro_export]
macro_rules! x {
    // x (A) => number represented by fixed length of A bits
    ( $size:literal ) => { $crate::BitNumber<$size> };
    ( [$size:literal] ) => { core::option::Option<x!($size)> };

    // x (i) => Variable Length Integer
    ( i ) => { $crate::Number };
    ( [i] ) => { core::option::Option<x!(i)> };

    // x (A..B) => number represented by A to B bits
    ( $min:literal..$max:literal ) => { $crate::BitRange<$min, $max> };
    ( [$min:literal..$max:literal] ) => { core::option::Option<x!($min..$max)> };

    // x (..B) => number represented by 0 to B bits
    ( ..$max:literal ) => { $crate::BitRange<0, $max> };
    ( [..$max:literal] ) => { core::option::Option<x!(..$max)> };

    // x (A..) => number represented by A to B bits
    ( $min:literal.. ) => { $crate::BitRange<$min> };
    ( [$min:literal..] ) => { core::option::Option<x!($min..)> };

    // x (..) => number represented by A to B bits
    ( .. ) => { $crate::BitRange };
    ( [..] ) => { core::option::Option<x!(..)> };

    // x (A) = C => number represented by fixed length of A bits with const value C
    ( $size:literal = $val:literal ) => { $crate::BitNumber<$size, $val, $val> };
    ( [$size:literal = $val:literal] ) => { core::option::Option<x!($size = $val)> };

    // x (A) = B..C => number represented by fixed length of A bits with value [B; C]
    ( $size:literal = $min:literal..$max:literal ) => { $crate::BitNumber<$size, $min, $max> };
    ( [$size:literal = $min:literal..$max:literal] ) => { core::option::Option<x!($size = $min..$max)> };

    // x (A) ... => any number of the other formats
    ( $size:literal; ...) => { Vec<x!($size)> };
    ( [$size:literal; ...] ) => { core::option::Option<x!($size; ...)> };
}

// ugly way to lock certain arms behind a feature
#[cfg(feature = "moq")]
#[macro_export]
macro_rules! x {
    // x (A) => number represented by fixed length of A bits
    ( $size:literal ) => { $crate::BitNumber<$size> };
    ( [$size:literal] ) => { core::option::Option<x!($size)> };

    // x (i) => Variable Length Integer
    ( i ) => { $crate::Number };
    ( [i] ) => { core::option::Option<x!(i)> };

    // x (A..B) => number represented by A to B bits
    ( $min:literal..$max:literal ) => { $crate::BitRange<$min, $max> };
    ( [$min:literal..$max:literal] ) => { core::option::Option<x!($min..$max)> };

    // x (..B) => number represented by 0 to B bits
    ( ..$max:literal ) => { $crate::BitRange<0, $max> };
    ( [..$max:literal] ) => { core::option::Option<x!(..$max)> };

    // x (A..) => number represented by A to B bits
    ( $min:literal.. ) => { $crate::BitRange<$min> };
    ( [$min:literal..] ) => { core::option::Option<x!($min..)> };

    // x (..) => number represented by A to B bits
    ( .. ) => { $crate::BitRange };
    ( [..] ) => { core::option::Option<x!(..)> };

    // x (A) = C => number represented by fixed length of A bits with const value C
    ( $size:literal = $val:literal ) => { $crate::BitNumber<$size, $val, $val> };
    ( [$size:literal = $val:literal] ) => { core::option::Option<x!($size = $val)> };

    // x (A) = B..C => number represented by fixed length of A bits with value [B; C]
    ( $size:literal = $min:literal..$max:literal ) => { $crate::BitNumber<$size, $min, $max> };
    ( [$size:literal = $min:literal..$max:literal] ) => { core::option::Option<x!($size = $min..$max)> };

    // x (A) ... => any number of the other formats
    ( $size:literal; ...) => { Vec<x!($size)> };
    ( [$size:literal; ...] ) => { core::option::Option<x!($size; ...)> };

    // x (b) => VarInt followed by that many bytes
    ( b ) => { $crate::BinaryData };
    ( [b] ) => { core::option::Option<x!(b)> };

    // x (tuple) => VarInt followed by that many x (b)
    ( tuple ) => { $crate::Tuple };
    ( [tuple] ) => { core::option::Option<x!(tuple)> };
}

#[cfg(test)]
mod tests {
    #[test]
    fn x_test() {
        let _bit = <x!(8)>::default();
        let _num = <x!(i)>::default();
        let _range = <x!(5..10)>::default();
        let _range = <x!(..10)>::default();
        let _range = <x!(5..)>::default();
        let _range = <x!(..)>::default();
        let _const = <x!(8 = 16)>::default();
        let _range = <x!(8 = 5..100)>::default();
        let _vec = <x!(8; ...)>::default();

        // optionals
        let _bit = <x!([8])>::default();
        let _num = <x!([i])>::default();
        let _range = <x!([5..10])>::default();
        let _range = <x!([..10])>::default();
        let _range = <x!([5..])>::default();
        let _range = <x!([..])>::default();
        let _const = <x!([8 = 16])>::default();
        let _range = <x!([8 = 5..100])>::default();
        let _vec = <x!([8; ...])>::default();

        #[cfg(feature = "moq")]
        let _binary = <x!(b)>::default();
        #[cfg(feature = "moq")]
        let _tuple = <x!(tuple)>::default();

        // optionals
        #[cfg(feature = "moq")]
        let _binary = <x!([b])>::default();
        #[cfg(feature = "moq")]
        let _tuple = <x!([tuple])>::default();
    }
}
