### Example Usage

```
cargo build
stap --dyninst probes/nmi.stap -o /dev/stderr -c 'target/debug/bobomb ~/roms/test.nes'
```
