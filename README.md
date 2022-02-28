## esp-fs-tests

To run the test suite on a esp32c3, run the following command. Make sure `cargo-espflash` is installed.

```bash
cargo +nightly espflash --features native --partition-table partitions.csv  --monitor /dev/ttyUSB0
```