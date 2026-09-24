use std::process::Command;

fn run_cli(args: &[&str]) -> String {
    let bin = env!("CARGO_BIN_EXE_taraskevizer-cli");
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to run CLI");
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim_end_matches('\n').to_string()
}

#[test]
fn test_cli_no_color_planeta() {
    assert_eq!(run_cli(&["--no-color", "планета"]), "плянэта");
}

#[test]
fn test_cli_no_color_all_caps() {
    assert_eq!(run_cli(&["--no-color", "ПЛАНЕТА"]), "ПЛАНЕТА");
}

#[test]
fn test_cli_no_color_no_escape_caps() {
    assert_eq!(
        run_cli(&["--no-color", "--no-escape-caps", "ПЛАНЕТА"]),
        "ПЛЯНЭТА"
    );
}

#[test]
fn test_cli_no_color_variations_all() {
    assert_eq!(run_cli(&["--no-color", "гродна"]), "(гродна|горадня)");
}

#[test]
fn test_cli_no_color_variations_no() {
    assert_eq!(
        run_cli(&["--no-color", "--no-variations", "гродна"]),
        "гродна"
    );
}

#[test]
fn test_cli_no_color_variations_first() {
    assert_eq!(
        run_cli(&["--no-color", "--first-variation", "гродна"]),
        "горадня"
    );
}

#[test]
fn test_cli_no_color_itoj_default() {
    assert_eq!(run_cli(&["--no-color", "яна і ён"]), "яна і ён");
}

#[test]
fn test_cli_no_color_itoj_always() {
    assert_eq!(
        run_cli(&["--no-color", "--jalways", "яна і ён"]),
        "яна й ён"
    );
}

#[test]
fn test_cli_no_color_latin() {
    assert_eq!(run_cli(&["--no-color", "-l", "планета"]), "planeta");
}

#[test]
fn test_cli_latin_ansi() {
    let result = run_cli(&["-l", "планета"]);
    assert_eq!(result, "p\u{1b}[32mlan\u{1b}[0meta");
}

#[test]
fn test_cli_no_color_energy() {
    assert_eq!(run_cli(&["--no-color", "энергія"]), "энэрґія");
}

#[test]
fn test_cli_no_color_disable_g() {
    assert_eq!(run_cli(&["--no-color", "--h", "энергія"]), "энэргія");
}

#[test]
fn test_cli_default_ansi_energy() {
    let result = run_cli(&["энергія"]);
    assert_eq!(result, "эн\x1b[32mэ\x1b[0mр\x1b[35mґ\x1b[0mія");
}

#[test]
fn test_cli_disable_g_ansi_energy() {
    let result = run_cli(&["--h", "энергія"]);
    assert_eq!(result, "эн\x1b[32mэ\x1b[0mр\x1b[35mг\x1b[0mія");
}

#[test]
fn test_cli_html_energy() {
    let result = run_cli(&["--html", "энергія"]);
    assert_eq!(result, "эн<tarF>э</tarF>р<tarH>ґ</tarH>ія");
}

#[test]
fn test_cli_html_disable_g_energy() {
    let result = run_cli(&["--html", "--h", "энергія"]);
    assert_eq!(result, "эн<tarF>э</tarF>р<tarH>г</tarH>ія");
}

#[test]
fn test_cli_alphabet_only_latin_ji() {
    let result = run_cli(&["--no-color", "--alphabet-only", "--latin-ji", "яна і іншыя"]);
    assert_eq!(result, "jana j jinšyja");
}

#[test]
fn test_cli_alphabet_only_latin() {
    let result = run_cli(&["--no-color", "--alphabet-only", "-l", "планета"]);
    assert_eq!(result, "płanieta");
}

// ===== Short-form args (matching JS parse-args.ts) =====

#[test]
fn test_cli_short_nc_planeta() {
    assert_eq!(run_cli(&["-nc", "планета"]), "плянэта");
}

#[test]
fn test_cli_short_nc_all_caps() {
    assert_eq!(run_cli(&["-nc", "ПЛАНЕТА"]), "ПЛАНЕТА");
}

#[test]
fn test_cli_short_nc_nec() {
    assert_eq!(run_cli(&["-nc", "-nec", "ПЛАНЕТА"]), "ПЛЯНЭТА");
}

#[test]
fn test_cli_short_nc_grodna() {
    assert_eq!(run_cli(&["-nc", "гродна"]), "(гродна|горадня)");
}

#[test]
fn test_cli_short_nc_nv() {
    assert_eq!(run_cli(&["-nc", "-nv", "гродна"]), "гродна");
}

#[test]
fn test_cli_short_nc_fv() {
    assert_eq!(run_cli(&["-nc", "-fv", "гродна"]), "горадня");
}

#[test]
fn test_cli_short_nc_itoj() {
    assert_eq!(run_cli(&["-nc", "яна і ён"]), "яна і ён");
}

#[test]
fn test_cli_short_nc_ja() {
    assert_eq!(run_cli(&["-nc", "-ja", "яна і ён"]), "яна й ён");
}

#[test]
fn test_cli_short_nc_l() {
    assert_eq!(run_cli(&["-nc", "-l", "планета"]), "planeta");
}

#[test]
fn test_cli_short_nc_energy() {
    assert_eq!(run_cli(&["-nc", "энергія"]), "энэрґія");
}

#[test]
fn test_cli_short_nc_h() {
    assert_eq!(run_cli(&["-nc", "--h", "энергія"]), "энэргія");
}

#[test]
fn test_cli_short_html_energy() {
    let result = run_cli(&["-html", "энергія"]);
    assert_eq!(result, "эн<tarF>э</tarF>р<tarH>ґ</tarH>ія");
}

#[test]
fn test_cli_short_html_h() {
    let result = run_cli(&["-html", "--h", "энергія"]);
    assert_eq!(result, "эн<tarF>э</tarF>р<tarH>г</tarH>ія");
}

#[test]
fn test_cli_short_abc_lj() {
    let result = run_cli(&["-nc", "-abc", "-lj", "яна і іншыя"]);
    assert_eq!(result, "jana j jinšyja");
}

#[test]
fn test_cli_short_abc_l() {
    let result = run_cli(&["-nc", "-abc", "-l", "планета"]);
    assert_eq!(result, "płanieta");
}

#[test]
fn test_cli_short_ph() {
    // phonetic mode — raw conversion, планета stays as-is (not taraskovized)
    assert_eq!(run_cli(&["-nc", "-ph", "планета"]), "планета");
}

#[test]
fn test_cli_short_st() {
    // single-thread — should produce same output as default
    assert_eq!(run_cli(&["-nc", "-st", "планета"]), "плянэта");
}

#[test]
fn test_cli_short_jr() {
    // jrandom — non-deterministic, just verify it runs without error
    let result = run_cli(&["-nc", "-jr", "яна і ён"]);
    // Should be either "яна і ён" or "яна й ён"
    assert!(
        result == "яна і ён" || result == "яна й ён",
        "jrandom should produce one of the valid forms, got: {result}"
    );
}
