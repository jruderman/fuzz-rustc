// Reasoning about the time complexity of the rust compiler
// * Identify patterns known to be slow, in order to notice when something is *unexpectedly* slow
// * Keep the fuzzer from getting bogged down with too many slow testcases

// Small file, but like with the nope module, I want this frequently-changing stuff separate and nonconflicting

use std::time::Duration;

pub fn expected_dur(input: &str) -> Option<Duration> {
    let size = input.len() as u128;
    let ibytes = input.as_bytes();

    let ct_ampers = ibytes.iter().filter(|b| **b == b'&').count() as u128;
    let ct_bang = ibytes.iter().filter(|b| **b == b'!').count() as u128;
    let ct_ques = ibytes.iter().filter(|b| **b == b'?').count() as u128;

    let highest_nesting_level = highest_nesting_normal_delims(input);

    // let longest_line_len ........... (https://github.com/rust-lang/rust/issues/103429 diagnostic arrow messes) (when not error-format=short)

    if
        // Only fully exclude things that are _frequently_, _annoyingly_ slow *and* we're confident that the
        // we have a way of excluding (some) hits in a way that doesn't significantly decrease fuzzer coverage

        // https://github.com/rust-lang/rust/issues/103421
        ct_ampers > 300 ||

        // desugars into nested matches
        ct_ques > 100 ||

        // Const eval can be arbitrarily slow if the limit is set high enough
        // (hmm, it might be better to mangle this instead of excluding it)
        // Revisit after #103877 lands
        input.contains("const_eval_limit") ||

        // https://github.com/rust-lang/rust/issues/117624
        (input.match_indices("dyn").count() > 5 && input.match_indices("Fn").count() > 5 && highest_nesting_normal_delims(&input) > 5) ||

        // https://github.com/rust-lang/rust/issues/104871
        (input.contains("||") && input.match_indices("let").count() > 10) ||

        // Nested macros can cause syntactic blowups:
        //     src/test/ui/issues/issue-65131.rs (10^3)
        (input.contains("macro_rules") && unique_macro_levels(input) > 3) ||

        // Similarly, nested derives can cause syntactic blowups:
        //     #[derive(...)] struct Y where B: Q::<{ /* nest */ }]
        (input.match_indices("derive").count() > 3 && unique_macro_levels(input) > 3) ||

        // Even non-nested macros can cause blowups in the type system:
        //     src/test/ui/enum/issue-42747.rs (4^27) (#104162)
        (input.contains("macro_rules") && ct_bang > 7)
    {
        eprintln!("timecpx is recommending to skip this input");
        None
    } else {
        let allowance_us = {
            // All programs
            size * size / 4 +  // small allowance for general quadraticness (~4s for a testcase at the soft limit of 4096 bytes)
            5_000 * size + // target 5ms per byte. linear is where we want the compiler to spend time... just not too much
            1_000_000 + // generous allowance for fixed overhead and measurement issues (system load, clock oddities)

            // Programs with specific weirdness. Based on measurements on Jesse's laptop. Times Fudge factor of 5.

            // Actually worse than quadratic, but we nope'd out early for high ampers counts
            // measured at 1us per ampers squared
            // https://github.com/rust-lang/rust/issues/103421
            5 * ct_ampers * ct_ampers +

            // Various exponential or quadratic-and-slow things that just aren't that interesting
            // These numbers are arbitrary
            200 * 2_u128.pow(highest_nesting_level.min(40) as u32)
        };

        // Mul some factors like computer speed and whether debug assertions are enabled?
        // Slower getting started? Is it the sancov?

        if allowance_us > 30_000_000 {
            eprintln!(
                "timecpx is concerned this testcase could take {} seconds",
                (allowance_us as f64) / 1_000_000.0
            );
            None
        } else {
            Some(Duration::from_micros(allowance_us as u64))
        }
    }
}

pub fn check_dur(elapsed: Duration, allowance: Duration) {
    if elapsed > allowance {
        eprintln!(
            "Compile time of {:?} exceeded the input-dependent allowance of {:?}",
            elapsed, allowance
        );
        // Stash it somewhere? With a sha hash??
    }
}

fn highest_nesting_normal_delims(input: &str) -> usize {
    let mut highest_nest_level: usize = 0;
    let mut current_nest_level: usize = 0;
    for b in input.as_bytes() {
        match b {
            b'(' | b'[' | b'{' => {
                current_nest_level += 1;
                if highest_nest_level < current_nest_level {
                    highest_nest_level = current_nest_level;
                }
            }
            b')' | b']' | b'}' => {
                current_nest_level = current_nest_level.saturating_sub(1);
            }
            _ => {}
        }
    }
    highest_nest_level
}

fn unique_macro_levels(input: &str) -> usize {
    let mut current_nest_level: usize = 0;
    let mut macro_at_level = vec![false];
    for b in input.as_bytes() {
        match b {
            b'(' | b'[' | b'{' => {
                current_nest_level += 1;
                if current_nest_level >= macro_at_level.len() {
                    macro_at_level.push(false);
                }
            }
            b')' | b']' | b'}' => {
                current_nest_level = current_nest_level.saturating_sub(1);
            }
            b'#' | b'!' => {
                macro_at_level[current_nest_level] = true;
            }
            _ => {}
        }
    }
    macro_at_level.iter().filter(|x| **x).count()
}
