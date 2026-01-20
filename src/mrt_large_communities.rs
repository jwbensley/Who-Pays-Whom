pub mod large_communities {
    use std::vec::Vec;

    use bgpkit_parser::models::LargeCommunity;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct LargeCommunities {
        large_communities: Vec<LargeCommunity>,
    }

    impl Default for LargeCommunities {
        fn default() -> Self {
            Self::new(Vec::<LargeCommunity>::new())
        }
    }

    impl LargeCommunities {
        pub fn new(large_communities: Vec<LargeCommunity>) -> Self {
            LargeCommunities { large_communities }
        }
    }
}
