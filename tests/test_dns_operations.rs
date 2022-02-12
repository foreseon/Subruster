#[cfg(test)]
mod test_dns_operations {
    #[tokio::test]
    async fn test_lookup() {
        let ns: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::new(8, 8, 8, 8));
        let host: String = "halborn.com".to_string();

        assert_eq!(subruster::dns_operations::lookup(Some(&[ns]), host.clone()).await.unwrap(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(151, 101, 2, 159)));
    }
}
