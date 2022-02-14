
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::{Instant};
use std::net::IpAddr;

use crate::http_operations;
use crate::dns_operations;
use crate::report;
use crate::args;

#[derive(Clone)]
pub struct Session {
    subdomains_found: Vec<String>,
    resolved_subdomains: Vec<String>,
    resolved_ips: Vec<IpAddr>,
    unresolved_subdomains: Vec<String>,
    subdomains_http_https: Vec<String>,
    useragent: String
}

impl Session {
    pub fn add_subdomains_found(&mut self, subdomain: String) {
        self.subdomains_found.push(subdomain);
    }
    pub fn get_subdomains_found(&self) -> Vec<String> {
        self.subdomains_found.clone()
    }
    pub fn set_subdomains_found(&mut self, subdomains_found: Vec<String>) {
        self.subdomains_found = subdomains_found;
    }

    pub fn get_resolved_subdomains(&self) -> Vec<String> {
        self.resolved_subdomains.clone()
    }
    pub fn add_resolved_subdomains(&mut self, subdomain: String) {
        self.resolved_subdomains.push(subdomain);
    }
    pub fn set_resolved_subdomains(&mut self, resolved_subdomains: Vec<String>) {
        self.resolved_subdomains = resolved_subdomains;
    }

    pub fn get_resolved_ips(&self) -> Vec<IpAddr> {
        self.resolved_ips.clone()
    }
    pub fn add_resolved_ips(&mut self, ip: IpAddr) {
        self.resolved_ips.push(ip);
    }
    pub fn set_resolved_ips(&mut self, resolved_ips: Vec<IpAddr>) {
        self.resolved_ips = resolved_ips;
    }

    pub fn get_unresolved_subdomains(&self) -> Vec<String> {
        self.unresolved_subdomains.clone()
    }
    pub fn add_unresolved_subdomains(&mut self, subdomain: String) {
        self.unresolved_subdomains.push(subdomain);
    }
    pub fn set_unresolved_subdomains(&mut self, unresolved_subdomains: Vec<String>) {
        self.unresolved_subdomains = unresolved_subdomains;
    }

    pub fn get_subdomains_http_https(&self) -> Vec<String> {
        self.subdomains_http_https.clone()
    }
    pub fn add_subdomains_http_https(&mut self, subdomain: String) {
        self.subdomains_http_https.push(subdomain);
    }
    pub fn set_subdomains_http_https(&mut self, subdomains_http_https: Vec<String>) {
        self.subdomains_http_https = subdomains_http_https;
    }

    pub fn set_useragent(&mut self, useragent: String) {
        self.useragent = useragent;
    }
    pub fn get_useragent(&self) -> String {
        self.useragent.clone()
    }

    pub fn init(useragent: String) -> Session {
        Session {
            subdomains_found: Vec::<String>::new(),
            resolved_subdomains: Vec::<String>::new(),
            resolved_ips: Vec::<IpAddr>::new(),
            unresolved_subdomains: Vec::<String>::new(),
            subdomains_http_https: Vec::<String>::new(),
            useragent: useragent,
        }
    }

    pub fn load(dns_subdomain_list: Vec<String>, resolved_subdomains: Vec<String>, resolved_ips: Vec<IpAddr>, unresolved_subdomains: Vec<String>, http_subdomain_list: Vec<String>, useragent: String) -> Session {
        Session {
            subdomains_found: dns_subdomain_list,
            resolved_subdomains: resolved_subdomains,
            resolved_ips: resolved_ips,
            unresolved_subdomains: unresolved_subdomains,
            subdomains_http_https: http_subdomain_list,
            useragent: useragent,
        }
    }
}

pub async fn start_session_operations() -> std::io::Result<()> {

    let start = Instant::now();

    let (session_args, useragentlist) = args::read_args();
    let mut current_session: Session = Session::init(session_args.get_current_useragent());
    let nameserver = session_args.get_nameserver();
    current_session.add_subdomains_found(session_args.get_hostname());
    current_session.set_useragent(session_args.get_current_useragent());
    let verbose = session_args.get_verbose_mode();
    let random_agent_in_every_req = session_args.get_random_agent_in_every_req();
    let http_timeout = session_args.get_http_timeout();

    //Start dns bruteforce
    if session_args.get_dnsbruteforce_mode() {
        println!("\x1b[1m\x1b[40mDNS BRUTEFORCE\x1b[0m");
        let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(session_args.get_dnsthread_number().try_into().unwrap())
        .build()
        .unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        let file = File::open(session_args.get_subdomain_txt_path())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let tx = tx.clone();
            let subdomain_to_search = format!("{}.{}", line.unwrap(), session_args.get_hostname());
            pool.spawn(move || {
                if call_dns_lookup(nameserver, &subdomain_to_search) == subdomain_to_search {
                    println!("Found subdomain: {}\x1b[0m   \x1b[1m(DNS bruteforce)\x1b[0m" , subdomain_to_search);
                    tx.send(subdomain_to_search);
                } 

            });            
        }
        drop(tx);

        let vecsubs: Vec<String> = rx.into_iter().collect();
        for x in 0..vecsubs.len() {
            if vecsubs[x] != "not_found"
            {
                current_session.add_subdomains_found(vecsubs[x].clone());
            }
        }
        println!();
    }
    //End dns bruteforce
    
    //Start internet search
    if session_args.get_searchengine_mode() {
        println!("\x1b[1m\x1b[40mINTERNET SEARCH\x1b[0m");
        let subdomain_list_internet_search : Vec<String> = http_operations::search_internet(&session_args.get_hostname(), &current_session.get_useragent());
        
        for x in 0..subdomain_list_internet_search.len() {
            if !current_session.get_subdomains_found().contains(&subdomain_list_internet_search[x])
            {
                current_session.add_subdomains_found(subdomain_list_internet_search[x].clone());
                println!("Found subdomain: {}\x1b[0m   \x1b[1m(Internet search)\x1b[0m" , subdomain_list_internet_search[x]);
            }
        }
        println!();
    }
    // End internet search

    //Start HTTP content search
    if session_args.get_httpsearch_mode() {
        println!("\x1b[1m\x1b[40mRECURSIVE HTTP CONTENT SEARCH\x1b[0m");
        println!("Sending requests...");
        let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(session_args.get_httpthread_number().try_into().unwrap())
        .build()
        .unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        let mut current_subdomains : Vec<String> = current_session.get_subdomains_found();
        //First the function sends the http requests with threads, finds all subdomains available inside http content (previously discovered or not) then joins this discovered list of domains with current subdomain list makes sort and dedup to create the new unique subdomain list.
        for subdomain in current_subdomains.clone() {
            let tx = tx.clone();
            let mut thread_session = current_session.clone();
            let thread_useragentlist = useragentlist.clone();
            pool.spawn(move || {
                if random_agent_in_every_req {
                    thread_session.set_useragent(thread_useragentlist.get_random_useragent());
                }
                
                let (subdomain_list_http_content_search, http_https_url) = call_http_content_search(&subdomain, &thread_session.get_useragent(), http_timeout, verbose);
                tx.send(subdomain_list_http_content_search);
            });  
        }
        drop(tx);
        let vec_vecsubs: Vec<Vec<String>> = rx.into_iter().collect();

        let mut new_subdomain_list : Vec<String> = current_subdomains.clone();
        for vector in vec_vecsubs {
            for subdomain in vector {
                new_subdomain_list.push(subdomain);
            }
        }
        new_subdomain_list.sort();
        new_subdomain_list.dedup();

        current_session.set_subdomains_found(new_subdomain_list.clone());

        let mut subdomain_difference_list: Vec<String> = new_subdomain_list.into_iter().filter(|item| !current_subdomains.contains(item)).collect();

        for subdomain in subdomain_difference_list.clone() {
            println!("Found subdomain: {}\x1b[0m   \x1b[1m(HTTP content search)\x1b[0m" , subdomain);
        }

        //After thread execution, function calculates the difference in subdomains before http content search and after. Function starts a recursive http content search on new found domains.
        let mut x_counter = 0;
        while x_counter < subdomain_difference_list.len() {

            if random_agent_in_every_req {
                current_session.set_useragent(useragentlist.get_random_useragent());
            }

            let (subdomain_list_http_content_search, http_https_url) = call_http_content_search(&subdomain_difference_list[x_counter], &current_session.get_useragent(), http_timeout, verbose);

            for y in 0..subdomain_list_http_content_search.len() {
                if !current_session.get_subdomains_found().contains(&subdomain_list_http_content_search[y])
                {
                    current_session.add_subdomains_found(subdomain_list_http_content_search[y].clone());
                    subdomain_difference_list.push(subdomain_list_http_content_search[y].clone());
                    println!("Found subdomain: {}\x1b[0m   \x1b[1m(HTTP content search)\x1b[0m" , subdomain_list_http_content_search[y]);
                }
            }
            x_counter += 1;
        }
        
        println!();
    }
    //End HTTP content search

    //Print results    
    let duration = start.elapsed();
    
    let mut current_session: Session = resolve_enumerated_subdomains(session_args.get_nameserver(), current_session, session_args.get_dnsthread_number()).await;
    
    if session_args.get_log_http_https_domains() {
        let webservice_url_list : Vec<String> = http_operations::find_webservice_available_urls(current_session.get_resolved_subdomains(), &current_session.get_useragent(), session_args.get_httpthread_number());
        current_session.set_subdomains_http_https(webservice_url_list);
    }

    let current_session = report::print_result(current_session);
    println!("\x1b[92mTime elapsed: {:?}\x1b[0m", duration);

    if session_args.get_report_mode() {
    report::create_report(session_args, current_session);
    }
    
    Ok(())
}

fn call_dns_lookup(nameserver: IpAddr, hostname: &String) -> String {
    let result = dns_operations::hostname_lookup_return_ip(nameserver, &hostname);
    match result {
        Ok(_n) => return hostname.to_string(),
        Err(n) => return n.to_string(),
    }
}

fn call_http_content_search(url: &String, useragent: &String, timeout: u64, verbose: bool) -> (Vec<String>,String) {
    http_operations::send_http_https_parse_response(url, useragent, timeout, verbose)
}


async fn resolve_enumerated_subdomains(nameserver: IpAddr, mut session: Session, thread_num: u64) -> Session {
    print!("Resolving found domains");

    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(thread_num.try_into().unwrap())
    .build()
    .unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    let (tx1, rx1) = std::sync::mpsc::channel();
    let (tx2, rx2) = std::sync::mpsc::channel();

        for subdomain in session.get_subdomains_found() {
            let tx = tx.clone(); //resolved subdomain
            let tx1 = tx1.clone(); //resolved ip
            let tx2 = tx2.clone(); //unresolved subdomain

            pool.spawn(move || {
                match dns_operations::hostname_lookup_return_ip(nameserver, &subdomain) {
                    Ok(n) =>  {
                        //session.add_resolved_subdomains(session.get_subdomains_found()[x].clone());
                        tx.send(subdomain.clone());
                        tx1.send(n);
                        //session.add_resolved_ips(n);
                    },
                    Err(e) => { 
                        //session.add_unresolved_subdomains(session.get_subdomains_found()[x].clone());
                        tx2.send(subdomain.clone());
                    },
                }
            });  

        }

        drop(tx);
        drop(tx1);
        drop(tx2);

        session.set_resolved_subdomains(rx.into_iter().collect());
        session.set_resolved_ips(rx1.into_iter().collect());
        session.set_unresolved_subdomains(rx2.into_iter().collect());


    println!();
    session
}


 