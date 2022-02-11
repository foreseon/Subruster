use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::{Error, ErrorKind};
use anyhow::{anyhow, Result};
use trust_dns_resolver::{ AsyncResolver, config::*};

#[tokio::main]
pub async fn hostname_lookup_print(nameserver: IpAddr, hostname: &String, verbose_mode: bool) -> std::io::Result<()> {
    let notfound_error = Error::new(ErrorKind::Other, "not_found");
    let address = lookup(Some(&[nameserver]), hostname.clone()).await;
    //println!("{}", hostname);
    match address {
        Ok(n) => {
                println!("Found subdomain: {}\x1b[0m   \x1b[1m(DNS bruteforce)\x1b[0m" , hostname);
                if verbose_mode {
                match n {
                    IpAddr::V4(ip4) => print!("  ipv4: {}", ip4),
                    IpAddr::V6(ip6) => print!("  ipv6: {}", ip6)
                }
                println!();
            }
        },
        Err(e) => {return Err(notfound_error);},
    }
    
    Ok(())
}

pub async fn lookup(nameservers: Option<&[IpAddr]>, host: String) -> Result<IpAddr> {
    let resolver = match nameservers {
        Some(nameservers) => {
            let nameserver_group = NameServerConfigGroup::from_ips_clear(nameservers, 53);
            AsyncResolver::tokio(ResolverConfig::from_parts(None, vec![], nameserver_group), ResolverOpts::default()).await?
        },
        None => AsyncResolver::tokio_from_system_conf().await?
    };
    let response = resolver.lookup_ip(host).await?;
    match response.iter().next() {
        Some(address) => Ok(address),
        None => Err(anyhow!("not found lookup address"))
    }
}

/*
pub fn hostname_lookup(hostname: String) -> (Vec<SocketAddr>,bool)  {

    let mut server: Vec<_> = vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)];
    let mut owned_hostname : String = hostname.to_owned();
    let port : String = ":80".to_owned();

    owned_hostname.push_str(&port);

    match owned_hostname.to_socket_addrs() {
        Ok(n) => server = n.collect(),
        Err(err) => return (server, false),
    };

    (server,true)


}

pub fn hostname_lookup_print(hostname: &String, verbose_mode: bool) -> std::io::Result<()>  {

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let notfound_error = Error::new(ErrorKind::Other, "not_found");
    let response = resolver.lookup_ip(hostname.clone());

    match response.unwrap() {
        LookupIp => {
            println!("\x1b[93mFound subdomain: {}\x1b[0m   \x1b[1m(DNS bruteforce)\x1b[0m" , hostname);
            if verbose_mode {
                for ips in response.iter() {
                    match ips {
                        IpAddr::V4(ip4) => print!(" | ipv4: {}", ip4),
                        IpAddr::V6(ip6) => print!(" | ipv6: {}", ip6),
                    }
                } 
                println!(" |\n------------------------------------------------------");
            }    
        
        },
        Err(err) => return Err(notfound_error),
    };   
    //println!("{}", hostname);
    Ok(())
}

pub fn hostname_lookup(hostname: &String)-> (Vec<IpAddr>, bool) {
    
    let mut ips: Vec<IpAddr> = vec![IpAddr::V4(Ipv4Addr::new(0,0,0,0))];
    match lookup_host(hostname) {
        Ok(n) => ips = n,
        Err(err) => return (ips, false),
    };

    (ips, true)
}

pub fn hostname_lookup_print(hostname: &String, verbose_mode: bool) -> std::io::Result<()> {
    let (ips, result) = hostname_lookup(&hostname);
    let notfound_error = Error::new(ErrorKind::Other, "not_found");
    //println!("{}", hostname);
    if result {
        println!("\x1b[93mFound subdomain: {}\x1b[0m   \x1b[1m(DNS bruteforce)\x1b[0m" , hostname);
            if verbose_mode {
                for ips in ips.iter() {
                    match ips {
                        IpAddr::V4(ip4) => print!(" | ipv4: {}", ip4),
                        IpAddr::V6(ip6) => print!(" | ipv6: {}", ip6),
                    }
                } 
                println!(" |\n------------------------------------------------------");
            }       
    }
    else {return Err(notfound_error);}
    
    Ok(())

}*/