#[cfg(test)]
mod test_args {
    #[test]
    fn test_read_useragents() {
        let mut test_list: subruster::args::UserAgentList = subruster::args::UserAgentList::init();
        assert_eq!(test_list.read_useragents().unwrap(), ());
    }
}
