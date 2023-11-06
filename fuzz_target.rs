#![feature(new_uninit)]
#![feature(read_buf)]
#![no_main]
#[macro_use] extern crate libfuzzer_sys;
mod mutator;
mod tst_mutator;
mod nope;
mod timecpx;
use std::io::{self, Write};
use std::time::Instant;

/// rustc_driver::Callbacks object that stops before codegen.
pub struct FuzzCallbacks;

impl rustc_driver::Callbacks for FuzzCallbacks {
    fn after_analysis<'tcx>(&mut self,
                            _early: &rustc_session::EarlyErrorHandler,
                            _compiler: &rustc_interface::interface::Compiler,
                            _queries: &'tcx rustc_interface::Queries<'tcx>,) -> rustc_driver::Compilation {
        // Stop before codegen.
        rustc_driver::Compilation::Stop
    }
}

/// FileLoader that holds the contents of a single input file in memory.
/// The idea here is to avoid needing to write to disk.
struct FuzzFileLoader {
    // The contents of the single input file.
    input: Vec<u8>,
}

impl FuzzFileLoader {
    fn new(input: Vec<u8>) -> Self {
        FuzzFileLoader {
            input,
        }
    }
}

// The name of the single input file.
const INPUT_PATH: &str = "fuzz_input.rs";

impl rustc_span::source_map::FileLoader for FuzzFileLoader {
    fn file_exists(&self, path: &std::path::Path) -> bool {
        std::path::Path::new(INPUT_PATH) == path
    }

    fn read_file(&self, path: &std::path::Path) -> std::io::Result<String> {
        if self.file_exists(path) {
            let s = match String::from_utf8(self.input.to_vec()) {
                Ok(s) => s,
                Err(_e) => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                          "not utf8")),
            };
            Ok(s)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "tried to open nonexistent file".to_string()))
        }
    }

    fn read_binary_file(&self, path: &std::path::Path) -> std::io::Result<rustc_data_structures::sync::Lrc<[u8]>> {
        use rustc_data_structures::sync::Lrc;
        use std::io::Read;
        if self.file_exists(path) {
            let len = self.input.len();
            let mut bytes = Lrc::new_uninit_slice(len as usize);
            let mut buf = std::io::BorrowedBuf::from(Lrc::get_mut(&mut bytes).unwrap());
            let mut file = std::io::Cursor::new(&self.input[..]);
            file.read_buf_exact(buf.unfilled())?;
            Ok(unsafe { bytes.assume_init() })
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "tried to open nonexistent file".to_string()))
        }
    }
}

/// CodegenBackend that panics when being called to do any actual codegen.
/// We use this to avoid needing to compile rustc_codegen_llvm.
pub struct NullCodegenBackend;

impl rustc_codegen_ssa::traits::CodegenBackend for NullCodegenBackend {
    fn codegen_crate<'tcx>(&self,
                           _: rustc_middle::ty::TyCtxt<'tcx>,
                           _: rustc_metadata::EncodedMetadata,
                           _: bool) -> std::boxed::Box<(dyn core::any::Any + 'static)> {
        unimplemented!()
    }

    fn join_codegen(
        &self,
        _ongoing_codegen: Box<dyn core::any::Any>,
        _sess: &rustc_session::Session,
        _outputs: &rustc_session::config::OutputFilenames,
    ) -> Result<(rustc_codegen_ssa::CodegenResults,
                 rustc_data_structures::fx::FxIndexMap<rustc_middle::dep_graph::WorkProductId,
                                                      rustc_middle::dep_graph::WorkProduct>),
                rustc_errors::ErrorGuaranteed> {
        unimplemented!()
    }

    fn link(
        &self,
        _sess: &rustc_session::Session,
        _codegen_results: rustc_codegen_ssa::CodegenResults,
        _outputs: &rustc_session::config::OutputFilenames,
    ) -> Result<(), rustc_errors::ErrorGuaranteed> {
        unimplemented!()
    }

    fn locale_resource(&self) -> &'static str {
        ""
    }
}


fn rustc_args(input: &str) -> Vec<String> {
    let mut v = vec![
        "rustc".to_string(),
        INPUT_PATH.to_string(),
          "-o".to_string(),
          "dummy_output_file".to_string(),
        ];

    // Pass through certain valid compile flags in the input. (In regression tests, these use e.g. "compile-flags:")

    if input.contains("--edition=2015") || input.contains("edition:2015") {
        v.push("--edition=2015".to_string());
    }
    else if input.contains("--edition=2018") || input.contains("edition:2018") {
        v.push("--edition=2018".to_string());
    }
    else {
        // Always specify some edition
        v.push("--edition=2021".to_string());
    }

    if input.contains("--diagnostic-width=20") {
        v.push("--diagnostic-width=20".to_string());
    }

    if input.contains("--error-format json") || input.contains("--error-format=json") {
        v.push("--error-format=json".to_string());
    }
    else if input.contains("--error-format short") || input.contains("--error-format=short") {
        v.push("--error-format=short".to_string());
    }

    // const eval limit: Revisit after #103877 lands
    v.push("-Zcrate_attr=feature(const_eval_limit)".to_string());
    v.push("-Zcrate_attr=const_eval_limit=\"1000\"".to_string());

    // recursion limit: down to 24 for #104225 and #105269; down to 14 for #104230
    v.push("-Zcrate_attr=recursion_limit=\"14\"".to_string());

    //v.push("-Zverbose".to_string());
    v.push("-L".to_string());
    v.push(env!("FUZZ_RUSTC_LIBRARY_DIR").to_string());
    v
}

pub fn main_fuzz(input: String) {
    println!("\n```rust\n{}\n```\n\n", &input);
    io::stdout().flush().unwrap();
    let args = rustc_args(&input);
    let file_loader = Box::new(FuzzFileLoader::new(input.into()));
    let mut callbacks = FuzzCallbacks;
    let _result = rustc_driver::catch_fatal_errors(|| {
        let mut run_compiler = rustc_driver::RunCompiler::new(&args, &mut callbacks);
        run_compiler.set_file_loader(Some(file_loader));
        run_compiler.set_make_codegen_backend(
            Some(Box::new(|_| {Box::new(NullCodegenBackend)})));
        run_compiler.run()
    }).and_then(|result| result);
}

fuzz_target!(|data: &[u8]| {
    if false {
        tst_mutator::exercise_mutator();
        println!("Bye");
        std::process::abort();
    }
    if data.contains(&0x0c) || data.contains(&0x0d) || data.contains(&0x0b) /*|| data.contains (&b'&')*/ {
        return;
    }
    if let Ok(t) = String::from_utf8(data.into()) {
        if nope::do_not_compile(&t) {
            return;
        }
        if let Some(allowance) = timecpx::expected_dur(&t) {
            let start = Instant::now();
            main_fuzz(t);
            timecpx::check_dur(start.elapsed(), allowance);
        }
    }
});

fuzz_mutator!(|data: &mut [u8], size: usize, max_size: usize, seed: u32| {
    use rand::{rngs::StdRng, Rng, SeedableRng};
    match String::from_utf8(data[0..size].to_vec()) {
        Ok(original_source) => {
            let mut rng = StdRng::seed_from_u64(seed as u64);
            if rng.gen_bool(0.9) && original_source.is_ascii() && !nope::do_not_even_parse(&original_source) {
                //println!("Custom mutator");
                let mu = mutator::ProgramMutator::new(original_source);
                let new_source = mu.random_mutation(&mut rng).unwrap();
                let new_data = new_source.as_bytes();
                let new_size = std::cmp::min(max_size, new_data.len());
                data[..new_size].copy_from_slice(&new_data[..new_size]);
                new_data.len()
            } else {
                //println!("Basic mutator");
                //Todo: for non-ascii files, sometimes try the 'reduction' of normalizing the file to ASCII
                libfuzzer_sys::fuzzer_mutate(data, size, max_size)
            }
        }
        _ => {
            // No changes: mutator can't handle it and rustc probably rejects it early anyway
            //eprintln!("How did this non-UTF-8 get in here?");
            //println!("No changes");
            size
        }
    }
});
