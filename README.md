# Subruster
Fast, compact and all-around subdomain enumeration tool written in Rust, which uses dns bruteforce, internet search and recursive http content search.
       
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
        
        -t, --threads <thread number>: Specifies the number of threads for dns bruteforce module
        
        --nohttp: Disables http content search module
        
        --nointernet: Disables internet search module
        
        --nodnsbrute: Disables dns bruteforce module
        
        --report-folder <report folder>: Specifies the output report folder (default is ./reports)
        
        -v, --verbose: Verbose mode
        
        --useragent <useragent>: Specifies the useragent in http requests
        
        --randomagent: Uses a random agent
        
        --randomagent-everyrequest: Uses different useragent in each http request
        
        --loghttp: Logs the subdomains which have http or https open
        
        -h, --help: This page
        
