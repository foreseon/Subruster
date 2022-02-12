use rand::Rng;
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use crate::file_operations;

///Command-line arguments saved to an Args struct after read/parsed
#[derive(Clone)]
pub struct Args {
    hostname: String,
    nameserver: IpAddr,
    httpsearch_mode: bool,
    http_timeout: u64,
    searchengine_mode: bool,
    thread_number: u32,
    dnsbruteforce_mode: bool,
    report_mode: bool,
    subdomain_txt_path: String,
    verbose_mode: bool,
    log_http_https_domains: bool,
    report_folder_path: String,
    random_agent_in_every_req: bool,
    current_useragent: String
}

impl Args {
    fn init() -> Args {
        Args {
            hostname: "".to_string(),
            nameserver: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            httpsearch_mode: true,
            http_timeout: 6,
            thread_number: 250,
            searchengine_mode: true,
            dnsbruteforce_mode: true,
            subdomain_txt_path: "./files/dnspod-top2000-sub-domains.txt".to_string(),
            verbose_mode: false,
            log_http_https_domains: false,
            report_mode: true,
            report_folder_path: "./report".to_string(),
            random_agent_in_every_req: false,
            current_useragent: "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/44.0.2403.125 Safari/537.36".to_string(),
        }
    }

    pub fn get_hostname(&self) -> String {
        self.hostname.clone()
    }
    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = hostname;
    }

    pub fn get_nameserver(&self) -> IpAddr {
        self.nameserver
    }
    pub fn set_nameserver(&mut self, nameserver: String) {
        self.nameserver = nameserver.parse().unwrap();
    }

    pub fn get_thread_number(&self) -> u32 {
        self.thread_number
    }
    pub fn set_thread_number(&mut self, thread_number: String ) {
        let t_n = thread_number.parse().unwrap();
        self.thread_number = t_n;
    }

    pub fn get_httpsearch_mode(&self) -> bool {
        self.httpsearch_mode.clone()
    }
    pub fn set_httpsearch_mode(&mut self, httpsearch_mode: bool ) {
        self.httpsearch_mode = httpsearch_mode;
    }

    pub fn get_http_timeout(&self) -> u64 {
        self.http_timeout.clone()
    }
    pub fn set_http_timeout(&mut self, http_timeout: u64 ) {
        self.http_timeout = http_timeout;
    }

    pub fn get_searchengine_mode(&self) -> bool {
        self.searchengine_mode.clone()
    }
    pub fn set_searchengine_mode(&mut self, searchengine_mode: bool ) {
        self.searchengine_mode = searchengine_mode;
    }

    pub fn get_dnsbruteforce_mode(&self) -> bool {
        self.dnsbruteforce_mode.clone()
    }
    pub fn set_dnsbruteforce_mode(&mut self, dnsbruteforce_mode: bool ) {
        self.dnsbruteforce_mode = dnsbruteforce_mode;
    }

    pub fn get_subdomain_txt_path(&self) -> String {
        self.subdomain_txt_path.clone()
    }
    pub fn set_subdomain_txt_path(&mut self, subdomain_txt_path: String ) {
        self.subdomain_txt_path = subdomain_txt_path;
    }

    pub fn get_verbose_mode(&self) -> bool {
        self.verbose_mode.clone()
    }
    pub fn set_verbose_mode(&mut self, verbose_mode: bool ) {
        self.verbose_mode = verbose_mode;
    }

    pub fn get_report_mode(&self) -> bool {
        self.verbose_mode.clone()
    }
    pub fn set_report_mode(&mut self, report_mode: bool ) {
        self.report_mode = report_mode;
    }

    pub fn get_log_http_https_domains(&self) -> bool {
        self.log_http_https_domains
    }
    pub fn set_log_http_https_domains(&mut self, log_http_https_domains: bool) {
        self.log_http_https_domains = log_http_https_domains;
    }
    
    pub fn get_report_folder_path(&self) -> String {
        self.report_folder_path.clone()
    }
    pub fn set_report_folder_path(&mut self, report_folder_path: String ) {
        self.report_folder_path = report_folder_path;
    }

    pub fn get_random_agent_in_every_req(&self) -> bool {
        self.random_agent_in_every_req.clone()
    }
    pub fn set_random_agent_in_every_req(&mut self, random_agent_in_every_req: bool ) {
        self.random_agent_in_every_req = random_agent_in_every_req;
    }

    pub fn get_current_useragent(&self) -> String {
        format!("{}", self.current_useragent)
    }
    pub fn set_current_useragent(&mut self, current_useragent: String ) {
        self.current_useragent = current_useragent;
    }
}
///Used to hold useragents in a list in order to chance useragents dynamically during execution
#[derive(Clone)]
pub struct UserAgentList {
    useragents: Vec<String>
}

impl UserAgentList {
    pub fn init() -> UserAgentList {
        UserAgentList {
            useragents: Vec::<String>::new(),
        }
    }

    ///Reads useragent list file line by line and saves it into the UserAgentList useragents vector
    pub fn read_useragents(&mut self)-> std::io::Result<()> {
        let mut reader = file_operations::BufReader::open("./files/useragent-list.txt")?;
        let mut buffer = String::new();
    
        while let Some(line) = reader.read_line(&mut buffer) {
            self.useragents.push(line?.trim().to_string());
        }
    
        Ok(())
    }

    pub fn get_random_useragent(&self) -> String {
        let mut rng = rand::thread_rng();
        let agent_file_size = self.useragents.len();
        self.useragents[rng.gen_range(0..agent_file_size-1)].clone()
    }

    pub fn get_useragent_list(&self) -> Vec<String> {
        self.useragents.clone()
    }
}


///Read and parse the command-line arguments
pub fn read_args() -> (Args, UserAgentList)  {
    let mut session_args: Args = Args::init();

    let mut hostname_flag: bool = false;
    let mut subdomain_txt_path_flag: bool = false;
    let mut report_folder_path_flag: bool = false;
    let mut current_useragent_flag: bool = false;
    let mut nameserver_flag: bool = false;
    let mut thread_flag: bool = false;
    let mut http_timeout_flag: bool = false;

    let mut useragentlist: UserAgentList = UserAgentList::init();

    println!("\x1b[1m{}\x1b[0m", print_logo());

    // Loop over arguments.
    for argument in env::args() {
    

        // If "-h" or "--help" detected, print help and exit.
        if argument == "-h" || argument == "--help"  {
            println!("{}", help());
            std::process::exit(0);
        }

        // Use flag after "-d" or "--domain" was detected to set argument.
        if hostname_flag {
            session_args.set_hostname(argument);
            hostname_flag = false;
            continue;
        }
        // If "-d" or "--domain" detected, set hostname flag bool.
        if argument == "-d" || argument == "--domain"  {
            hostname_flag = true;
            continue;
        }
        // If "--nohttpsearch" detected, set httpsearch_mode to false.
        if argument == "--nohttp"  {
            session_args.set_httpsearch_mode(false);
            continue;
        }

        // If "--nosearchengine" detected, set searchengine_mode to false.
        if argument == "--nointernet" {
            session_args.set_searchengine_mode(false);
            continue;
        }

        // If "--nodnsbrute" detected, set dnsbruteforce_mode to false.
        if argument == "--nodnsbrute" {
            session_args.set_dnsbruteforce_mode(false);
            continue;
        }

        // If "--loghttp" detected, set dnsbruteforce_mode to true.
        if argument == "--loghttp" {
            session_args.set_log_http_https_domains(true);
            continue;
        }

        // Use flag after "--subdomain-wordlist" or "-w" was detected to set argument.
        if subdomain_txt_path_flag {
            session_args.set_subdomain_txt_path(argument);
            subdomain_txt_path_flag = false;
            continue;
        }
        // If "--subdomain-file" detected, set hostname flag bool.
        if argument == "--subdomain-wordlist" || argument == "-w"  {
            subdomain_txt_path_flag = true;
            continue;
        }


        // Use flag after "--subdomain-wordlist" or "-w" was detected to set argument.
        if http_timeout_flag {
            session_args.set_http_timeout(argument.parse().unwrap());
            http_timeout_flag = false;
            continue;
        }
        // If "--httptimeout" detected, set hostname flag bool.
        if argument == "--httptimeout" {
            http_timeout_flag = true;
            continue;
        }

        // If "-v" or "--verbose" detected, set verbose_mode to false.
        if argument == "-v" || argument == "--verbose" {
            session_args.set_verbose_mode(true);
            continue;
        }

        // Use flag after "--report-output" was detected to set argument.
        if report_folder_path_flag {
            session_args.set_report_folder_path(argument);
            report_folder_path_flag = false;
            continue;
        }
        // If "--report-output" detected, set hostname flag bool.
        if argument == "--report-folder" {
            report_folder_path_flag = true;
            continue;
        } 

        // If "--randomagent" detected, read useragent file into the UserAgentList struct and choose a random agent for current useragent.
        if argument == "--randomagent" {
            useragentlist.read_useragents();
            session_args.set_current_useragent(useragentlist.get_random_useragent());
            continue;
        }

        // If "--randomagent-everyrequest" detected, set dnsbruteforce_mode to false.
        if argument == "--randomagent-everyrequest" {
            session_args.set_random_agent_in_every_req(true);
            continue;
        }

       // Use flag after "--useragent" was detected to set argument.
        if current_useragent_flag {
            session_args.set_current_useragent(argument);
            current_useragent_flag = false;
            continue;
        }
        // If "--useragent" detected, set current_useragent_flag flag bool.
        if argument == "--useragent" {
            current_useragent_flag = true;
            continue;
        }

        // Use flag after "--nameserver" or "-ns" was detected to set argument.
        if nameserver_flag {
            session_args.set_nameserver(argument);
            nameserver_flag = false;
            continue;
        }
        // If "--nameserver" or "-ns" detected, set nameserver_flag flag bool.
        if argument == "--nameserver" || argument == "-ns"  {
            nameserver_flag = true;
            continue;
        }

        // Use flag after "--threads" or "-t" was detected to set argument.
        if thread_flag {
            session_args.set_thread_number(argument);
            thread_flag = false;
            continue;
        }
        // If "--threads" or "-t" detected, set thread_flag flag bool.
        if argument == "--threads" || argument == "-t"  {
            thread_flag = true;
            continue;
        }
        
    }

    // Display arguments.
    println!("{}", display_args(&session_args));

    (session_args, useragentlist)
    
}

fn display_args(args: &Args) -> String {
    format!("
    [+] Target domain: {}

    [+] Dns bruteforce: {}
        -subdomain wordlist: {}
        -nameserver: {}
        -thread number: {}

    [+] Internet search: {}

    [+] Recursive http content search: {}
        -request timeout: {}
        -user-agent: {}
        -random user-agent in every-req: {}

    [+] Log subdomains with http/s: {}

    [+] Reporting: {}
        -report folder: {}

    Verbose: {}    
    ", args.get_hostname(), args.get_dnsbruteforce_mode().to_string(), args.get_subdomain_txt_path(), args.get_nameserver(), args.get_thread_number().to_string(), args.get_searchengine_mode().to_string(), args.get_httpsearch_mode().to_string(), args.get_http_timeout().to_string(), args.get_current_useragent(), args.get_random_agent_in_every_req().to_string(), args.get_log_http_https_domains().to_string(), args.get_report_mode().to_string(), args.get_report_folder_path() ,args.get_verbose_mode().to_string()
)
}

fn help() -> String {
    format!(" 

    Subruster v1.0 (https://github.com/foreseon/Subruster)

    Subruster is a fast, compact and all-around subdomain enumeration tool written in Rust, which uses dns bruteforce, internet search and recursive http content search.
    
    DNS BRUTEFORCE MODULE:
    Takes a wordlist of subdomains and perform a dns query using threads.
    
    INTERNET SEARCH MODULE:
    Searches subdomains in dns.bufferover, crt.sh, dnsrepo (more to come.)
    
    RECURSIVE HTTP CONTENT SEARCH MODULE:
    Tries to connect to http and https ports of the domains and searches for subdomains inside the response html, executes recursively if any new subdomain is found during the process.
    
    Usage:
    
        -d, --domain <target domain>: Specifies the target domain
        
        -w, --subdomain-wordlist <subdomain wordlist path>: Specifies the subdomain wordlist path (default is ./file/subdomain-list-top2000)
        
        -ns, --nameserver <nameserver ip>: Specifies the nameserver (default is 8.8.8.8)
        
        -t, --threads <thread number>: Specifies the number of threads for dns bruteforce module (You may need to execute 'ulimit -n 999999' in your terminal if you want to work with big number of threads)
        
        --nohttp: Disables http content search module
        
        --nointernet: Disables internet search module
        
        --nodnsbrute: Disables dns bruteforce module
        
        --report-folder <report folder>: Specifies the output report folder (default is ./reports)
        
        -v, --verbose: Verbose mode

        --loghttp: Checks if subdomains have http/s services open and logs them
        
        --useragent <useragent>: Specifies the useragent in http requests
        
        --randomagent: Uses a random agent
        
        --randomagent-everyrequest: Uses different useragent in each http request
        
        -h, --help: This page
        
        ")
}

fn print_logo() -> String {
    format!("

    _______ _     _ ______   ______ _     _ _______ _______ _______  ______
    |______ |     | |_____] |_____/ |     | |______    |    |______ |_____/
    ______| |_____| |_____] |    \\_ |_____| ______|    |    |______ |    \\_
                                                                           
    ")
}