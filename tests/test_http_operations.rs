#[cfg(test)]
mod test_http_operations {
    #[test]
    fn test_send_http_req() {
        let host: String = "google.com".to_string();

        assert_ne!(subruster::http_operations::send_http_req(&host, 5, &"test".to_string() , false), ("".to_string(), false));
    }
    #[test]
    fn test_send_https_req() {
        let host: String = "google.com".to_string();

        assert_ne!(subruster::http_operations::send_https_req(&host, 5, &"test".to_string() , false), ("".to_string(), false));
    }
}
