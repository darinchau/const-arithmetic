### v1.0.4
Update macros to use literal invocation to avoid many copies of TypedInteger::new()
Improves test suite compile time performace by about 30%

### v1.0.3
Hides implementation details in docs.
Optimizes division with hexshift algorithm. 
Added `Copy` to all trait requirements

Here are the benchmarks details for division:
Command run with
```powershell
cargo check --tests --timings
```
Compile with 571 division (test.rs division only): (81m 04s new, 100m 59s old)
Bad news is it still currently takes around 8 seconds to check every division

### v1.0.2
Fixes license

### v1.0.0
Initial release with TypedInteger, Typed(Add, Sub, Mul, Div)
