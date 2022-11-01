// Some programs should not even be fed to the parser that's part of the mutator, much less the full rustc_driver
pub fn do_not_even_parse(prog: &str) -> bool {
    if might_nest_too_deep(prog) {
        //eprintln!("Nope: might nest too deep");
        return true;
    }
    false
}

// Making the parser recurse too deep will make it run out of stack space,
// which can show up as several types of exits (abort, sigsegv on a guard page, or worse)
// It can also lead to very large diagnostics if a single line is very long (https://github.com/rust-lang/rust/issues/103429)
// Also it's often just slow, without being so slow (e.g. exponential) that it's worth reporting a bug
// (RUST_MIN_STACK=20000000 wasn't enough to stop the crashes?)
fn might_nest_too_deep(prog: &str) -> bool {
    let mut ct = 0;
    for &byte in prog.as_bytes() {
        if byte == b'(' || byte == b'[' || byte == b'{' || byte == b'|' || byte == b'&' || byte == b'\"' {
            ct += 1
        }
    }
    ct > 400
}


pub fn do_not_compile(prog: &str) -> bool {
    if do_not_even_parse(prog) {
        return true;
    }
    if prog.contains("rustc_strict_coherence") {
        // Semi-intentional ICE: https://github.com/rust-lang/rust/issues/103753
        return true;
    }
    if prog.contains("BikeshedIntrinsicFrom") {
        // Let's give this nightly-only feature a rest for a while:
        // https://github.com/rust-lang/rust/issues/103634
        // https://github.com/rust-lang/rust/issues/103751
        // https://github.com/rust-lang/rust/issues/103783
        return true;
    }
    if prog.contains("generic_const_exprs") || prog.contains("adt_const_params") {
        // Let's give this nightly-only feature a rest for a while:
        // https://github.com/rust-lang/rust/issues/103770
        // https://github.com/rust-lang/rust/issues/103790 (even without the feature gate, but this still helps)
        return true;
    }

    false
}
