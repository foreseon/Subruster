use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use chrono;

use crate::session_manager::Session;
use crate::args::Args;
use crate::file_operations;

pub fn print_result(session: Session) -> Session {
    println!();
    println!("\x1b[1m\x1b[40m-----------RESULT-----------\x1b[0m");
    println!("\x1b[1m\x1b[91mUNRESOLVED DOMAINS - {}\x1b[0m", session.get_unresolved_subdomains().len());
    for x in 0..session.get_unresolved_subdomains().len() {
        println!("{}", session.get_unresolved_subdomains()[x]);
    }
    println!("\x1b[1m\x1b[92mRESOLVED DOMAINS - {}\x1b[0m", session.get_resolved_subdomains().len());
    for x in 0..session.get_resolved_subdomains().len() {
        print!("{}", session.get_resolved_subdomains()[x]);
        println!(" - {}", session.get_resolved_ips()[x]);
    }
    println!("\x1b[1m\x1b[92mSUBDOMAINS WITH HTTP/S SERVICE - {}\x1b[0m", session.get_subdomains_http_https().len());
    if session.get_subdomains_http_https().len() == 0 {
        println!("No HTTP/S subdomain found, enable httpcontent search if disabled")
    }
    for x in 0..session.get_subdomains_http_https().len() {
        println!("{}", session.get_subdomains_http_https()[x]);
    }
    println!();
    session
}

pub fn create_report(args: Args, session: Session)-> std::io::Result<()> {

    let mut report_file_path = args.get_report_folder_path().to_owned();
    report_file_path.push_str("/");
    report_file_path.push_str(&args.get_hostname());
    report_file_path.push_str("/");

    match file_operations::create_directory(&report_file_path) {
        Ok(n) => {},
        Err(e) => {
            println!("Unable to create report");
            std::process::exit(0);
        },
    }
    let datetime: String = chrono::offset::Local::now().to_string().to_owned();
    report_file_path.push_str("result-");
    report_file_path.push_str(&datetime);
    report_file_path.push_str(".txt");

    let mut f = File::create(report_file_path.clone()).expect("Unable to create report");
    let mut file = OpenOptions::new().write(true).append(true).open(report_file_path).unwrap();   

    writeln!(file, "---UNRESOLVED DOMAINS--- ({})", session.get_unresolved_subdomains().len().to_string());
    for i in &session.get_unresolved_subdomains() {                                                                                                                                                                  
        writeln!(file,"{}", i)?;                                                                                                                     
    }
    writeln!(file, "---RESOLVED DOMAINS--- ({})", session.get_resolved_subdomains().len().to_string());
    for i in &session.get_resolved_subdomains() {                                                                                                                                                                  
        writeln!(file,"{}", i)?;                                                                                                                     
    }
    writeln!(file, "---SUBDOMAINS WITH HTTP/S SERVICE--- ({})", session.get_subdomains_http_https().len().to_string());
    for i in &session.get_subdomains_http_https() {                                                                                                                                                                  
        writeln!(file,"{}", i)?;                                                                                                                     
    }
    Ok(())
}

