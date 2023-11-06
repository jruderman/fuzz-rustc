// Some programs should not even be fed to the parser that's part of the mutator, much less the full rustc_driver
pub fn do_not_even_parse(prog: &str) -> bool {
    if might_nest_too_deep(prog) {
        //eprintln!("Nope: might nest too deep");
        return true;
    }
    if highest_nesting_angle_brackets(prog) > 30 {
        // https://github.com/rust-lang/rust/issues/103620: temporarily here rather than in timecpx(?)
        return true;
    }
    if highest_nesting_curly(prog) > 12 && prog.match_indices(":").count() > 12 {
        // https://github.com/rust-lang/rust/issues/105067
        return true;
    }
    if prog.match_indices("#").count() > 21 && prog.match_indices("!").count() > 7 && prog.match_indices("=").count() > 7 && prog.match_indices("{").count() > 7 {
        // https://github.com/rust-lang/rust/issues/105165
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

fn highest_nesting_angle_brackets(input: &str) -> usize {
    let mut highest_nest_level: usize = 0;
    let mut current_nest_level: usize = 0;
    for b in input.as_bytes() {
        match b {
            b'<' => {
                current_nest_level += 1;
                if highest_nest_level < current_nest_level {
                    highest_nest_level = current_nest_level;
                }
            }
            b'>' => {
                current_nest_level = current_nest_level.saturating_sub(1);
            }
            _ => {}
        }
    }
    highest_nest_level
}


fn highest_nesting_curly(input: &str) -> usize {
    let mut highest_nest_level: usize = 0;
    let mut current_nest_level: usize = 0;
    for b in input.as_bytes() {
        match b {
            b'{' => {
                current_nest_level += 1;
                if highest_nest_level < current_nest_level {
                    highest_nest_level = current_nest_level;
                }
            }
            b'}' => {
                current_nest_level = current_nest_level.saturating_sub(1);
            }
            _ => {}
        }
    }
    highest_nest_level
}


pub fn do_not_compile(prog: &str) -> bool {
    if do_not_even_parse(prog) {
        return true;
    }
    if prog.contains("generic_const_exprs") || prog.contains("adt_const_params") {
        // https://github.com/rust-lang/rust/issues/103770
        // https://github.com/rust-lang/rust/issues/106994 (and all issues linked from there)
        return true;
    }
    if prog.contains("specialization") {
        // https://github.com/rust-lang/rust/issues/103708
        return true;
    }
    if prog.contains("rustc_peek") {
        // This intrinsic is expected to ICE if misused:
        // https://github.com/rust-lang/rust/issues/83461#issuecomment-1467596046
        return true;
    }
    false
}
