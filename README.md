# `pyth-layout-bpf-test`

From version `0.9.0` to `0.10.1` there was a change in the representation of some of the fields of `PriceAccount`/`SolanaPriceAccount`. Although the representations are identical on some x86 and arm machines, this repository demonstrates the layout differences for the bpf architecture.

## Usage

```
cargo-test-sbf
```
