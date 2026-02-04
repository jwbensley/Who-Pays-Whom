pub mod asn {
    use std::fmt;

    use bgpkit_parser::models::Asn;
    use serde::{Serialize, Serializer};

    static TIER1_ASNS: [u32; 17] = [
        174, 701, 1273, 1299, 2914, 3257, 3320, 3356, 3491, 5511, 6453, 6461, 6762, 6830, 6939,
        7018, 12956,
    ];

    // Skip ASNs which are contributing inaccurate/invalid data to route collectors.
    // Two tuples of ASN and route collector filenames, where the ASN is seen.
    static SKIP_ASNS: [(u32, &str); 6] = [
        (37468, "route-views.napafrica.rib.20260131.0000.bz2"),
        (37468, "route-views.ix-br2.gru.rib.20260131.0000.bz2"),
        (37468, "route-views2.saopaulo.rib.20260131.0000.bz2"),
        (37468, "route-views3.rib.20260131.0000.bz2"),
        (37468, "ris.rrc15.bview.20260131.0000.gz"),
        (37468, "ris.rrc19.bview.20260131.0000.gz"),
    ];

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct MrtAsn(Asn);

    impl fmt::Display for MrtAsn {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
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
            Self(asn)
        }

        pub fn from_u32(asn: u32) -> Self {
            Self::new(Asn::new_32bit(asn))
        }

        pub fn to_u32(self) -> u32 {
            self.0.to_u32()
        }

        /// Some ASNs seem to be sending incorrect data to route collectors
        pub fn is_skip_asn(&self, filename: &str) -> bool {
            SKIP_ASNS.contains(&(self.0.to_u32(), filename))
        }

        pub fn is_t1_asn(&self) -> bool {
            TIER1_ASNS.contains(&self.0.to_u32())
        }
    }
}
