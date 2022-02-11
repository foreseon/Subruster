use std::time::Duration;
use regex::Regex;

fn send_http_req(url: &String, timeout: u64, useragent: &String, verbose: bool) -> (String, bool) {
    let mut url_http : String = "http://".to_string();
    url_http.push_str(url);

    if verbose {
    println!("\x1b[90mSearching subdomains in: {} response\x1b[0m" , url_http);
    }

    let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true).timeout(Duration::from_secs(timeout)).build().unwrap();
    
    let body_http = client.get(url_http).header("User-Agent", useragent).send();
    
    match body_http {
            Ok(ref _n) => {},
            Err(e) => {
                if e.is_timeout() {
                    return ("".to_string(), false);
                }
                else if e.is_request() {
                    return ("".to_string(), false);
                }
                else if e.is_redirect() {
                    return ("".to_string(), true);
                }
                else if e.is_decode() {
                    return ("".to_string(), true);
                }
                return ("".to_string(), false);
            },
        }

        match body_http.unwrap().text() {
            Ok(n) => return (n, true),
            Err(_e) => return ("".to_string(), false),
        }
}
    
fn send_https_req(url: &String, timeout: u64, useragent: &String, verbose: bool) -> (String, bool) {
    let mut url_https : String = "https://".to_string();
    url_https.push_str(url);

    if verbose {
    println!("\x1b[90mSearching subdomains in: {} response\x1b[0m" , url_https);
    }

    let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true).timeout(Duration::from_secs(timeout)).build().unwrap();
    
    let body_https = client.get(url_https).header("User-Agent", useragent).send();

    match body_https {
            Ok(ref _n) => {},                
            Err(e) => {
                if e.is_timeout() {
                    return ("".to_string(), false);
                }
                else if e.is_request() {
                    return ("".to_string(), false);
                }
                else if e.is_redirect() {
                    return ("".to_string(), true);
                }
                else if e.is_decode() {
                    return ("".to_string(), true);
                }
                return ("".to_string(), false);
            },
        }


        match body_https.unwrap().text() {
            Ok(n) => return (n, true),
            Err(e) => return ("".to_string(), false),
        }
        
}


pub fn send_http_https_parse_response(url: &String, useragent: &String, verbose: bool) -> (Vec<String>, String) {
    let (response_http, is_http) = send_http_req(url, 4, useragent, verbose);
    let (response_https, is_https) = send_https_req(url, 4, useragent, verbose);

    let mut webserver_url = "".to_string();
    if is_https {
        webserver_url.push_str("https://");
        webserver_url.push_str(url);
    }
    else if is_http {
        webserver_url.push_str("http://");
        webserver_url.push_str(url);
    }

    let response = format!("{}{}", response_http, response_https);

    let mut subdomain_list : Vec<String> = Vec::<String>::new();

    let mut subdomain_regex_string1 = r"(?:http[s]*\\:\\/\\/)*([[:alnum:]]*?)\.".to_string();
    subdomain_regex_string1.push_str(url);

    let mut subdomain_regex_string2 = r"([[:alnum:]]*?)\.".to_string();
    subdomain_regex_string2.push_str(url);

    let regex1 = Regex::new(&subdomain_regex_string1).unwrap();
    let regex2 = Regex::new(&subdomain_regex_string2).unwrap();

    for subdomain in regex1.captures_iter(&response) {
        subdomain_list.push(subdomain[0].to_string());
    }

    for subdomain in regex2.captures_iter(&response) {
        subdomain_list.push(subdomain[0].to_string());
    }

    subdomain_list.sort();
    subdomain_list.dedup();
    
    (subdomain_list, webserver_url)
}

pub fn search_internet(hostname: &String, useragent: &String) -> Vec<String> {

    let mut subdomain_list : Vec<String> = Vec::<String>::new();

    subdomain_list.append(&mut search_bufferoverrun(hostname, useragent));
    subdomain_list.append(&mut search_crtsh(hostname, useragent));
    subdomain_list.append(&mut search_dnsrepo(hostname, useragent));
    //subdomain_list.append(&mut search_bing(hostname, 10, useragent));
    //subdomain_list.append(&mut search_yandex(hostname, 10, useragent));

    subdomain_list.sort();
    subdomain_list.dedup();

    subdomain_list

}

fn search_crtsh(hostname: &String, useragent: &String) -> Vec<String> {
    let mut url = "crt.sh/?q=".to_string().to_owned();
    url.push_str(hostname);
    url.push_str("&output=json");

    let (response, _webserver_url) = send_https_req(&url, 35, useragent, true);

    let mut subdomain_regex_string = r"([[:alnum:]]*?)\.".to_string();
    subdomain_regex_string.push_str(hostname);

    let regex = Regex::new(&subdomain_regex_string).unwrap();

    let mut subdomain_list : Vec<String> = Vec::<String>::new();
    for subdomain in regex.captures_iter(&response) {
        subdomain_list.push(subdomain[0].to_string());
    }
    
    subdomain_list.sort();
    subdomain_list.dedup();

    subdomain_list
}

fn search_bufferoverrun(hostname: &String, useragent: &String) -> Vec<String> {
    let mut url = "dns.bufferover.run/dns?q=".to_string().to_owned();
    url.push_str(hostname);

    let (response, _webserver_url) = send_https_req(&url, 35, useragent, true);

    let mut subdomain_regex_string = r"([[:alnum:]]*?)\.".to_string();
    subdomain_regex_string.push_str(hostname);

    let regex = Regex::new(&subdomain_regex_string).unwrap();

    let mut subdomain_list : Vec<String> = Vec::<String>::new();
    for subdomain in regex.captures_iter(&response) {
        subdomain_list.push(subdomain[0].to_string());
    }
    
    subdomain_list.sort();
    subdomain_list.dedup();

    subdomain_list
}

fn search_dnsrepo(hostname: &String, useragent: &String) -> Vec<String> {
    let mut url = "dnsrepo.noc.org/?domain=".to_string().to_owned();
    url.push_str(hostname);

    let (response, _webserver_url) = send_https_req(&url, 35, useragent, true);
    let mut subdomain_regex_string = r"([[:alnum:]]*?)\.".to_string();
    subdomain_regex_string.push_str(hostname);

    let regex = Regex::new(&subdomain_regex_string).unwrap();

    let mut subdomain_list : Vec<String> = Vec::<String>::new();
    for subdomain in regex.captures_iter(&response) {
        subdomain_list.push(subdomain[0].to_string());
    }
    
    subdomain_list.sort();
    subdomain_list.dedup();

    subdomain_list
}

fn search_bing(hostname: &String, useragent: &String, depth: u32) -> Vec<String> {
    
    let mut url = "".to_string();
    let mut subdomain_list : Vec<String> = Vec::<String>::new();

    for x in 0..depth {
        url = "www.bing.com/search?q=".to_string().to_owned();
        url.push_str(hostname);
        url.push_str("&first=");

        if x > 0 {
            let page_num: String = (x * 10).to_string().to_owned();
            url.push_str(&page_num.clone());            
        }
        else {
            url.push_str("1");
        }
            
        let (response, _webserver_url) = send_https_req(&url, 35, useragent, true);
        let mut subdomain_regex_string = r"([[:alnum:]]*?)\.".to_string();
        subdomain_regex_string.push_str(hostname);
    
        let regex = Regex::new(&subdomain_regex_string).unwrap();
    
        
        for subdomain in regex.captures_iter(&response) {
            subdomain_list.push(subdomain[0].to_string());
        }
    }

    
    subdomain_list.sort();
    subdomain_list.dedup();

    subdomain_list
}

fn search_yandex(hostname: &String, useragent: &String, depth: u32) -> Vec<String> {

    let mut url = "".to_string();
    let mut subdomain_list : Vec<String> = Vec::<String>::new();

    for x in 0..depth {

        url = "yandex.com.tr/search/?lr=1&text=".to_string().to_owned();
        url.push_str(hostname);
        url.push_str("&p=");
        url.push_str(&x.to_string());
            
        let (response, _webserver_url) = send_https_req(&url, 35, useragent, true);
        let mut subdomain_regex_string = r"([[:alnum:]]*?)\.".to_string();
        subdomain_regex_string.push_str(hostname);
    
        let regex = Regex::new(&subdomain_regex_string).unwrap();
    
        for subdomain in regex.captures_iter(&response) {
            subdomain_list.push(subdomain[0].to_string());
        }
    }

    subdomain_list.sort();
    subdomain_list.dedup();

    subdomain_list
}