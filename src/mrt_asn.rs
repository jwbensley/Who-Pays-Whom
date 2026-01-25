pub mod asn {
    use bgpkit_parser::models::Asn;
    use serde::{Serialize, Serializer};

    pub static TIER1_ASNS: [u32; 17] = [
        174, 701, 1273, 1299, 2914, 3257, 3320, 3356, 3491, 5511, 6453, 6461, 6762, 6830, 6939,
        7018, 12956,
    ];

    pub trait IsT1Asn {
        fn is_t1(&self) -> bool;
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct MrtAsn(Asn);

    impl IsT1Asn for MrtAsn {
        fn is_t1(&self) -> bool {
            TIER1_ASNS.contains(&self.0.to_u32())
        }
    }

    impl Serialize for MrtAsn {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u32(self.0.to_u32())
        }
    }

    impl MrtAsn {
        pub fn new(asn: Asn) -> Self {
            Self { 0: asn }
        }

        pub fn from_u32(asn: u32) -> Self {
            Self::new(Asn::new_32bit(asn))
        }

        pub fn to_u32(self) -> u32 {
            self.0.to_u32()
        }
    }
}
