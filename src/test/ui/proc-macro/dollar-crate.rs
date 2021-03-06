// edition:2018
// aux-build:dollar-crate.rs
// aux-build:dollar-crate-external.rs

// Anonymize unstable non-dummy spans while still showing dummy spans `0..0`.
// normalize-stdout-test "bytes\([^0]\w*\.\.(\w+)\)" -> "bytes(LO..$1)"
// normalize-stdout-test "bytes\((\w+)\.\.[^0]\w*\)" -> "bytes($1..HI)"

extern crate dollar_crate;
extern crate dollar_crate_external;

type S = u8;

mod local {
    use crate::dollar_crate;

    macro_rules! local {
        () => {
            dollar_crate::m! {
                struct M($crate::S);
            }

            #[dollar_crate::a]
            struct A($crate::S);

            #[derive(dollar_crate::d)]
            struct D($crate::S); //~ ERROR the name `D` is defined multiple times
        };
    }

    local!();
}

mod external {
    use crate::dollar_crate_external;

    dollar_crate_external::external!(); //~ ERROR the name `D` is defined multiple times
}

fn main() {}
