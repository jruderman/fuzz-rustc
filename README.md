# Fuzz Rustc

This repo contains configuration for fuzz-testing the Rust compiler using [libfuzzer-sys](https://github.com/rust-fuzz/libfuzzer),
taking inspiration from [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and [fuzz-targets](https://github.com/rust-fuzz/targets).

Because [rustc](https://github.com/rust-lang/rust) is a bootstrapping compiler, its build process has several stages
and involves juggling many flags, attributes, and environment variables. These complications create some difficulties for
cleanly setting up fuzz testing. We work around those difficulties with some
[light modifications to rustc](rust-changes.diff) and some additional configuration.


## Running


```sh
./run-fuzzer.sh
```

You may add some example inputs in the `./seeds/` directory.

New interesting test cases are automatically written to the `./corpus/` directory as they are found.

The run-fuzzer.sh script passes trailing arguments on to the underlying libfuzzer binary,
so you can pass any of these options: https://llvm.org/docs/LibFuzzer.html#options .

For example, this invocation will run 4 jobs in parallel and will only try ascii inputs:

```sh
./run_fuzzer.sh -jobs=4 -only_ascii=1
```

## Bugs found

[#62524](https://github.com/rust-lang/rust/issues/62524)
[#62546](https://github.com/rust-lang/rust/issues/62546)
[#62554](https://github.com/rust-lang/rust/issues/62554)
[#62863](https://github.com/rust-lang/rust/issues/62863)
[#62881](https://github.com/rust-lang/rust/issues/62881)
[#62894](https://github.com/rust-lang/rust/issues/62894)
[#62895](https://github.com/rust-lang/rust/issues/62895)
[#62913](https://github.com/rust-lang/rust/issues/62913)
[#62973](https://github.com/rust-lang/rust/issues/62973)
[#63116](https://github.com/rust-lang/rust/issues/63116)
[#63135](https://github.com/rust-lang/rust/issues/63135)
[#66473](https://github.com/rust-lang/rust/issues/66473)
[#68629](https://github.com/rust-lang/rust/issues/68629)
[#68730](https://github.com/rust-lang/rust/issues/68730)
[#68890](https://github.com/rust-lang/rust/issues/68890)
[#69130](https://github.com/rust-lang/rust/issues/69130)
[#69310](https://github.com/rust-lang/rust/issues/69310)
[#69378](https://github.com/rust-lang/rust/issues/69378)
[#69396](https://github.com/rust-lang/rust/issues/69396)
[#69401](https://github.com/rust-lang/rust/issues/69401)
[#69600](https://github.com/rust-lang/rust/issues/69600)
[#69602](https://github.com/rust-lang/rust/issues/69602)
[#70549](https://github.com/rust-lang/rust/issues/70549)
[#70552](https://github.com/rust-lang/rust/issues/70552)
[#70594](https://github.com/rust-lang/rust/issues/70594)
[#70608](https://github.com/rust-lang/rust/issues/70608)
[#70677](https://github.com/rust-lang/rust/issues/70677)
[#70724](https://github.com/rust-lang/rust/issues/70724)
[#70736](https://github.com/rust-lang/rust/issues/70736)
[#70763](https://github.com/rust-lang/rust/issues/70763)
[#70813](https://github.com/rust-lang/rust/issues/70813)
[#70942](https://github.com/rust-lang/rust/issues/70942)
[#71297](https://github.com/rust-lang/rust/issues/71297)
[#71471](https://github.com/rust-lang/rust/issues/71471)
[#71798](https://github.com/rust-lang/rust/issues/71798)
[#72410](https://github.com/rust-lang/rust/issues/72410)
[#84104](https://github.com/rust-lang/rust/issues/84104)
[#84117](https://github.com/rust-lang/rust/issues/84117)
[#84148](https://github.com/rust-lang/rust/issues/84148)
[#84149](https://github.com/rust-lang/rust/issues/84149)
[#86895](https://github.com/rust-lang/rust/issues/86895)
[#88770](https://github.com/rust-lang/rust/issues/88770)
[#89868](https://github.com/rust-lang/rust/issues/89868)
[#92267](https://github.com/rust-lang/rust/issues/92267)
[#102114](https://github.com/rust-lang/rust/issues/102114)
[#102751](https://github.com/rust-lang/rust/issues/102751)
[#102878](https://github.com/rust-lang/rust/issues/102878)
[#103143](https://github.com/rust-lang/rust/issues/103143)
[#103195](https://github.com/rust-lang/rust/issues/103195)
[#103202](https://github.com/rust-lang/rust/issues/103202)
[#103210](https://github.com/rust-lang/rust/issues/103210)
[#103219](https://github.com/rust-lang/rust/issues/103219)
[#103411](https://github.com/rust-lang/rust/issues/103411)
[#103421](https://github.com/rust-lang/rust/issues/103421)
[#103427](https://github.com/rust-lang/rust/issues/103427)
[#103429](https://github.com/rust-lang/rust/issues/103429)
[#103451](https://github.com/rust-lang/rust/issues/103451)
[#103497](https://github.com/rust-lang/rust/issues/103497)
[#103599](https://github.com/rust-lang/rust/issues/103599)
[#103620](https://github.com/rust-lang/rust/issues/103620)
[#103634](https://github.com/rust-lang/rust/issues/103634)
[#103708](https://github.com/rust-lang/rust/issues/103708)
[#103748](https://github.com/rust-lang/rust/issues/103748)
[#103751](https://github.com/rust-lang/rust/issues/103751)
[#103770](https://github.com/rust-lang/rust/issues/103770)
[#103771](https://github.com/rust-lang/rust/issues/103771)
[#103783](https://github.com/rust-lang/rust/issues/103783)
[#103790](https://github.com/rust-lang/rust/issues/103790)
[#103824](https://github.com/rust-lang/rust/issues/103824)
[#104140](https://github.com/rust-lang/rust/issues/104140)
[#104162](https://github.com/rust-lang/rust/issues/104162)
[#104172](https://github.com/rust-lang/rust/issues/104172)
[#104209](https://github.com/rust-lang/rust/issues/104209)
[#104213](https://github.com/rust-lang/rust/issues/104213)
[#104225](https://github.com/rust-lang/rust/issues/104225)
[#104230](https://github.com/rust-lang/rust/issues/104230)
[#104249](https://github.com/rust-lang/rust/issues/104249)
[#104277](https://github.com/rust-lang/rust/issues/104277)
[#104281](https://github.com/rust-lang/rust/issues/104281)
[#104287](https://github.com/rust-lang/rust/issues/104287)
[#104291](https://github.com/rust-lang/rust/issues/104291)
[#104312](https://github.com/rust-lang/rust/issues/104312)
[#104327](https://github.com/rust-lang/rust/issues/104327)
[#104328](https://github.com/rust-lang/rust/issues/104328)
[#104352](https://github.com/rust-lang/rust/issues/104352)
[#104367](https://github.com/rust-lang/rust/issues/104367)
[#104368](https://github.com/rust-lang/rust/issues/104368)
[#104412](https://github.com/rust-lang/rust/issues/104412)
[#104510](https://github.com/rust-lang/rust/issues/104510)
[#104513](https://github.com/rust-lang/rust/issues/104513)
[#104551](https://github.com/rust-lang/rust/issues/104551)
[#104562](https://github.com/rust-lang/rust/issues/104562)
[#104583](https://github.com/rust-lang/rust/issues/104583)
[#104609](https://github.com/rust-lang/rust/issues/104609)
[#104613](https://github.com/rust-lang/rust/issues/104613)
[#104620](https://github.com/rust-lang/rust/issues/104620)
[#104768](https://github.com/rust-lang/rust/issues/104768)
[#104769](https://github.com/rust-lang/rust/issues/104769)
[#104802](https://github.com/rust-lang/rust/issues/104802)
[#104808](https://github.com/rust-lang/rust/issues/104808)
[#104871](https://github.com/rust-lang/rust/issues/104871)
[#104916](https://github.com/rust-lang/rust/issues/104916)
[#105011](https://github.com/rust-lang/rust/issues/105011)
[#105047](https://github.com/rust-lang/rust/issues/105047)
[#105067](https://github.com/rust-lang/rust/issues/105067)
[#105069](https://github.com/rust-lang/rust/issues/105069)
[#105097](https://github.com/rust-lang/rust/issues/105097)
[#105101](https://github.com/rust-lang/rust/issues/105101)
[#110014](https://github.com/rust-lang/rust/issues/110014)
[#111148](https://github.com/rust-lang/rust/issues/111148)

## TODO

Generalize this setup to also work other fuzzing engines, like AFL and Honggfuzz.

## License

All files in this repository are licensed [CC0](https://creativecommons.org/publicdomain/zero/1.0/), except for rust-changes.diff, which is licensed like the code it modifies.
