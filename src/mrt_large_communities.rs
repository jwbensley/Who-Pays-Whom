pub mod large_communities {
    use std::vec::Vec;

    use bgpkit_parser::models::LargeCommunity;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct LargeCommunities {
        data: Vec<LargeCommunity>,
    }

    impl LargeCommunities {
        pub fn new(d: Vec<LargeCommunity>) -> Self {
            LargeCommunities { data: d }
        }
    }
}
