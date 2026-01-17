pub mod asn_data {
    use std::collections::HashMap;

    use bgpkit_parser::models::Asn;

    #[derive(Debug)]
    pub struct AsnData {
        data: HashMap<Asn, Asn>,
    }

    impl AsnData {
        pub fn new() -> Self {
            AsnData {
                data: HashMap::<Asn, Asn>::new(),
            }
        }
    }
}
