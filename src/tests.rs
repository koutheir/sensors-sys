#![cfg(all(test, target_os = "linux", not(target_env = "kernel")))]

#[test]
fn sensors_strerror() {
    let r = unsafe { super::sensors_strerror(super::SENSORS_ERR_IO) };
    assert!(!r.is_null());
}
