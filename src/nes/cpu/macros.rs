// Testing macros

macro_rules! assert_status_set {
    ($c:expr, $f:expr) => {
        assert!($c.SR.is_set($f), "expected '{:?}' status register to be set", $f);
    }
}

macro_rules! assert_status_reset {
    ($c:expr, $f:expr) => {
        assert!(!$c.SR.is_set($f), "expected '{:?}' status register to be reset", $f);
    }
}

macro_rules! assert_cpu_register {
    ($c:expr, $r:expr, $v:expr) => {
        let z = $c.register_value($r);
        assert!(z == $v, "expected '{:#x}', got {:#x} in register {:?}", $v, z, $r);
    }
}

macro_rules! assert_equalx {
    ($l:expr, $r:expr) => {
        assert!($l == $r, "expected {:#x} got {:#x}", $r, $l);
    }
}
