# Subruster

![Screenshot from 2022-02-11 14-00-17](https://user-images.githubusercontent.com/25774631/153580434-80ec9b03-4ad5-40e1-8ec5-36eaa0843180.png)

Fast, compact and all-around subdomain enumeration tool written in Rust, which uses dns bruteforce, internet search and recursive http content search.

Seperates resolved, unresolved and subdomains with webservices inside report.


https://user-images.githubusercontent.com/25774631/153847521-22aa46f7-4c22-43bb-919e-f00277b71712.mp4


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
        
        -dt, --dnsthread <thread number>: Specifies the number of threads for dns bruteforce module (You may need to execute 'ulimit -n 999999' in your terminal if you want to work with big number of threads)
        
        -ht, --httpthread: <thread number>: Specifies the number of threads for http content search module

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
