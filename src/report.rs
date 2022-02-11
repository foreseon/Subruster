use crate::session_manager::Session;

pub fn print_result(session: Session) {
    println!();
    println!("\x1b[1m\x1b[40m-----------RESULT-----------\x1b[0m");
    println!("\x1b[1m\x1b[91mUNRESOLVED DOMAINS\x1b[0m");
    for x in 0..session.get_unresolved_subdomains().len() {
        println!("{}", session.get_unresolved_subdomains()[x]);
    }
    println!("\x1b[1m\x1b[92mRESOLVED DOMAINS\x1b[0m");
    for x in 0..session.get_resolved_subdomains().len() {
        println!("{}", session.get_resolved_subdomains()[x]);
        println!("{}", session.get_resolved_ips()[x]);
    }
    println!("\x1b[1m\x1b[94mSUBDOMAINS WITH HTTP/S SERVICE\x1b[0m");
    if session.get_subdomains_http_https().len() == 0 {
        println!("No HTTP/S subdomain found, enable httpcontent search if disabled")
    }
    for x in 0..session.get_subdomains_http_https().len() {
        println!("{}", session.get_subdomains_http_https()[x]);
    }
    println!();
}

fn create_report()-> std::io::Result<()> {
    Ok(())
}

