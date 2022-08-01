#[macro_export]
macro_rules! vec_destr {
    { $v:expr => $( $n:ident : $i:expr; )+ } => {
        let ( $( $n ),+ ) = {
            let mut v = $v;
            (
                $( std::mem::take(&mut v[$i]) ), +
            )
        };
    }
}
