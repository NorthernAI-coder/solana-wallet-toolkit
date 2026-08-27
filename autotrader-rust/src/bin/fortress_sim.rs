use autotrader_fortress::run_adversarial_simulation;

fn main() {
    let iterations = std::env::args()
        .nth(1)
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1_000_000);

    let report = run_adversarial_simulation(iterations);

    println!("Autotrader Fortress adversarial simulation");
    println!("iterations: {}", report.iterations);
    println!("total policy checks: {}", report.total_checks());
    println!("valid entry checks: {}", report.valid_entry_checks);
    println!("invalid entry checks: {}", report.invalid_entry_checks);
    println!("valid exit checks: {}", report.valid_exit_checks);
    println!("emergency exit checks: {}", report.emergency_exit_checks);
    println!(
        "emergency stale/same-signal routes caught: {}",
        report.emergency_route_failures_caught
    );
    println!("false accepts: {}", report.false_accepts);
    println!("false rejects: {}", report.false_rejects);
    println!("status: {}", if report.passed() { "PASS" } else { "FAIL" });

    if !report.passed() {
        std::process::exit(1);
    }
}
