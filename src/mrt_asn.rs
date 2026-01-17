pub mod asn {
    use bgpkit_parser::models::Asn;

    pub trait Tier1Asn {
        fn is_t1(&self) -> bool;
    }

    impl Tier1Asn for Asn {
        fn is_t1(&self) -> bool {
            [
                174, 701, 1299, 2914, 3257, 3320, 3356, 3491, 5511, 6453, 6461, 6762, 6830, 6939,
                7018,
            ]
            .contains(&self.to_u32())
        }
    }
}
