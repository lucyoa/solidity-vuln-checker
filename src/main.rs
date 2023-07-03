use reqwest;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use serde_json::{Value};


fn download_and_save(url: &str, filepath: &str) {
    println!("[*] Downloading to {}", filepath);
    let client = reqwest::blocking::Client::new();
    let result = client.get(url).send();

    let body = result.expect("failed").text().unwrap();

    let mut file = File::create(filepath).unwrap();
    file.write_all(body.as_bytes()).unwrap();
}

fn update() {
    println!("Updating...");
    const BUGS_BY_VERSION_URL: &str = "https://raw.githubusercontent.com/ethereum/solidity/develop/docs/bugs_by_version.json";
    const BUGS_URL: &str = "https://raw.githubusercontent.com/ethereum/solidity/develop/docs/bugs.json";

    download_and_save(BUGS_BY_VERSION_URL, "db/bugs_by_version.json");
    download_and_save(BUGS_URL, "db/bugs.json");
}

fn print_bug(bug_data: &Value) {
    let ver_introduced = match bug_data["introduced"].as_str() {
        Some(v) => v,
        None => "*",
    };
    let ver_fixed = match bug_data["fixed"].as_str() {
        Some(v) => v,
        None => "*"
    };

    println!("# {} - {}", bug_data["uid"].as_str().unwrap(), bug_data["name"].as_str().unwrap());
    println!("### Severity: {}", bug_data["severity"].as_str().unwrap());
    println!("### Versions: {} - {}",  ver_introduced, ver_fixed);
    println!("### Summary\n{}", bug_data["summary"].as_str().unwrap());
    println!("### Description\n{}\n", bug_data["description"].as_str().unwrap());

}

fn version(ver: &str) {
    let mut file = File::open("db/bugs_by_version.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let data: Value = serde_json::from_str(&content).unwrap();
    let vulns_names: Vec<Value> = data[ver]["bugs"].as_array().expect("failed").to_vec();

    let mut bugs_content = String::new();
    file = File::open("db/bugs.json").unwrap();
    file.read_to_string(&mut bugs_content).unwrap();
    let bugs_json: Vec<Value> = serde_json::from_str(&bugs_content).unwrap(); 

    println!("Results...\n");
    for vuln_name in vulns_names {
        for bug_data in &bugs_json {
            if vuln_name == bug_data["name"] {
                print_bug(bug_data);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "update" {
        update();
    }
    else if args[1] == "version" {
        version(&args[2]);
    }
}
