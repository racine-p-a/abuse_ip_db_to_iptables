use std::env;
use std::process::{Command};
use regex::Regex;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_argument = &args[1]; // The token should be here.
        parse_ips(get_abuse_ipdb(first_argument));
    } else {
        println!("Usage : sudo ./iptables-abuse_IP_db YOUR_TOKEN_HERE");
    }


}

fn parse_ips(raw_ips: String){
    let ip_pattern = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    let ip_addresses_to_ban: Vec<&str> = ip_pattern.find_iter(&*raw_ips)
        .map(|mat| mat.as_str())
        .collect();
    for ip_address in ip_addresses_to_ban {
        let output = Command::new("iptables")
            .arg("-A")
            .arg("INPUT")
            .arg("-s")
            .arg(ip_address)
            .arg("-j")
            .arg("DROP")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Banned IP address: {}", ip_address);
                } else {
                    println!(
                        "Failed to ban IP address {}: {}",
                        ip_address,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                println!("Error executing iptables command: {}", e);
            }
        }
    }
}

fn get_abuse_ipdb(token: &str) -> String {
    let result = "Key:".to_string() + " " + token;
    let output = Command::new("curl")
        .args([
            "--get", "https://api.abuseipdb.com/api/v2/blacklist",
            "-d", "confidenceMinimum=75",
            "-d", "limit=9999999",
            "-H", result.as_str(),
            "-H", "Accept: application/json"
        ])
        .output()
        .expect("curl command failed to start");
    //println!("status: {}", output.status);
    //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
}
