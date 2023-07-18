#[macro_export]
macro_rules! if_chain {
    (($check:ident, $comp:tt), ($fval:literal $fres:literal), $(($val:literal $res:literal)),*, $def:literal) => {
        if $check $comp $fval {
            $fres
        }
        $(else if $check $comp $val { $res })*
        else {
            $def
        }
    };
}
