### Example Usage

```
cargo build
stap --dyninst probes/nmi.stap -o /dev/stderr -c 'target/debug/bobomb ~/roms/test.nes'
```

You can also precompile the module for multiple runs:

```
stap -p4 -m bobomb_memory_read -c 'target/debug/bobomb' probes/memory_read_write.stap
staprun -c 'target/debug/bobomb ~/roms/test.nes' -o /dev/stderr bobomb_memory_read.ko
```
