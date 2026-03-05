use anyhow::{Context, Result};
use dotenvy::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::blocking::Client;
use std::{env, fs, path::Path, thread, time::Duration, io::{self, Write}};

/// Implements a typewriter-style terminal output for visual feedback.
fn slow_print(text: &str, char_delay: u64, line_delay: u64) {
    for line in text.lines() {
        for c in line.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(char_delay));
        }
        println!();
        thread::sleep(Duration::from_millis(line_delay));
    }
}

/// Displays the system identification header and initialization sequence.
fn run_breach_protocol() {
    let cobol_code = r#"
******************************************************************
* SYSTEM: DGMAG-SCANNER-v4.0            AUTHOR: RACHEL           *
* SUBSYSTEM: ADAPTIVE-SUFFIX-PROBE      SECURITY-LVL: ROOT       *
******************************************************************
 IDENTIFICATION DIVISION.
 PROGRAM-ID. DGMAG-EXTRACTOR.

 PROCEDURE DIVISION.
 000-BOOT.
     DISPLAY "INITIATING DOUBLE-TAP PROBE..." UPON CYBER-DECK.
     DISPLAY "1. ATTEMPTING STANDARD FILENAME MATCHING."
     DISPLAY "2. TRIGGERING SUFFIX BRUTE-FORCE ON FAIL."
     PERFORM 100-CHECK-STATE.
     PERFORM 200-SCAN-ISSUES.
     DISPLAY "TASK COMPLETE. GOING DARK.".
******************************************************************"#;

    slow_print(cobol_code, 15, 120);
    println!();
}

/// Main entry point. Handles environment loading, issue discovery, 
/// and state persistence.
fn main() -> Result<()> {
    run_breach_protocol();
    dotenv().ok();

    let state_file = env::var("STATE_FILE").unwrap_or_else(|_| "dgmag_state.sys".to_string());
    let base_url = env::var("BASE_URL").expect("[!] ERROR: BASE_URL NOT SET");

    let mut last_issue = if Path::new(&state_file).exists() {
        fs::read_to_string(&state_file)?.trim().parse::<i32>().unwrap_or(-1)
    } else {
        -1
    };

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Fedora; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .build()?;

    let mut found_links = Vec::new();
    let mut current_idx = last_issue + 1;

    // Defined suffixes and extensions based on known archival patterns
    let suffixes = vec!["", "a", "b", "c", "d", "e", "pre", "a4"];
    let extensions = vec![".pdf", ".PDF"];

    println!("[*] SCANNER ENGAGED. STARTING FROM INDEX N{}...", current_idx);

    loop {
        let mut found_this_issue = false;

        // Nested iteration: testing all combinations of suffixes and extensions
        'suffix_search: for ext in &extensions {
            for suffix in &suffixes {
                let filename = format!("DowngradeN{}{}{}", current_idx, suffix, ext);
                // Check alternative naming convention (underscored) as seen in specific releases
                let alt_filename = format!("Downgrade_N{}{}{}", current_idx, suffix, ext);

                for target in &[filename, alt_filename] {
                    let url = format!("{}/N{}/{}", base_url, current_idx, target);
                    let resp = client.head(&url).send()?;

                    if resp.status().is_success() {
                        println!("[+] TARGET ACQUIRED: N{} -> {}", current_idx, target);
                        found_links.push((current_idx, url));
                        last_issue = current_idx;
                        found_this_issue = true;
                        break 'suffix_search; 
                    }
                }
            }
        }

        if !found_this_issue {
            println!("[*] INFO: Issue #{} not found (all patterns failed). Stopping.", current_idx);
            break;
        }

        current_idx += 1;
        thread::sleep(Duration::from_millis(150));
    }

    if found_links.is_empty() {
        println!("[!] NO NEW DATA. ENTERING STEALTH MODE.");
        return Ok(());
    }

    send_email(&found_links, last_issue > 0 && last_issue == found_links.len() as i32 - 1)?;

    fs::write(&state_file, last_issue.to_string())?;
    println!("[*] STATE UPDATED TO {}. MISSION COMPLETE.", last_issue);

    Ok(())
}

/// Constructs and transmits an SMTP message via the configured relay.
/// Generates a bash-compatible download script in the message body.
fn send_email(links: &[(i32, String)], is_archive: bool) -> Result<()> {
    let email_addr = env::var("EMAIL").expect("[!] ERROR: EMAIL NOT SET");
    let smtp_server = env::var("SMTP_SERVER").expect("[!] ERROR: SMTP_SERVER NOT SET");
    let password = env::var("SMTP_PASSWORD").context("[!] ERROR: SMTP_PASSWORD NOT SET")?;

    let subject = if is_archive { "DGMAG: FULL ARCHIVE EXFILTRATION" } else { "DGMAG: NEW INTERCEPT" };
    let mut body = String::from("ROOT ACCESS GRANTED. RACHEL'S SCANNER REPORT:\n\nIDENTIFIED PDF PAYLOADS:\n");

    let mut urls = Vec::new();
    for (num, url) in links {
        body.push_str(&format!("-> Downgrade Issue #{}: {}\n", num, url));
        urls.push(url.clone());
    }

    body.push_str("\n--- BASH COMMAND TO DOWNLOAD ---\n");
    if urls.len() == 1 {
        body.push_str(&format!("curl -L -O '{}'\n", urls[0]));
    } else {
        body.push_str("for url in ");
        for u in &urls { body.push_str(&format!("'{}' ", u)); }
        body.push_str("; do curl -L -O \"$url\"; done\n");
    }
    body.push_str("--------------------------------\n\nEND OF LINE.");

    let email = Message::builder()
        .from(email_addr.parse()?)
        .to(email_addr.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;

    let creds = Credentials::new(email_addr, password);
    let mailer = SmtpTransport::relay(&smtp_server)?.credentials(creds).build();
    mailer.send(&email)?;
    println!("[+] DATA SENT.");
    Ok(())
}
