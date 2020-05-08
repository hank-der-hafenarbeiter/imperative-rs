use trybuild;

mod encoding;
mod decoding;

#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/collisions/exact.rs");
    t.compile_fail("tests/collisions/prefix.rs");
    t.compile_fail("tests/collisions/variable_infix.rs");
    t.compile_fail("tests/collisions/variable_prefix.rs");
    t.compile_fail("tests/collisions/variable_suffix.rs");
    t.compile_fail("tests/collisions/mixed_bin_hex.rs");

    t.compile_fail("tests/variable_checks/missing_in_opcode.rs");
    t.compile_fail("tests/variable_checks/missing_field.rs");
    t.compile_fail("tests/variable_checks/hex_digit_var.rs");
    t.compile_fail("tests/variable_checks/two_symbol_varname.rs");
}