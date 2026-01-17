pub mod communities {
    use std::vec::Vec;

    use bgpkit_parser::models::Community;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Communities {
        data: Vec<Community>,
    }

    impl Communities {
        pub fn new(d: Vec<Community>) -> Self {
            Communities { data: d }
        }
    }
}
