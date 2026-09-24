#[test]
fn old_probe_tmp() {
    let cfg = taraskevizer_core::TaraskConfig::default();
    for input in ["a <<= b", "a , b", "1,2", "a!?b", "<Планета <<= Планета>", "вялікі &#40не)"] {
        eprintln!("{:?} => {:?}", input, taraskevizer_core::tarask(input, &cfg));
    }
}
