use crate::comm_mappings::community_mappings::CommMappings;
use crate::mrt_communities::standard_communities::StandardCommunity;
use crate::peer_data::peer_data::{PeerLocation, PeerType};
use bgpkit_parser::models::Asn;
use std::collections::HashMap;

pub fn insert_comm_mapping(asn_mappings: &mut HashMap<Asn, CommMappings>) {
    /* Cogent */
    asn_mappings.insert(
        Asn::new_32bit(174),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(174, 21000), PeerType::Peer),
                (StandardCommunity::new(174, 21001), PeerType::Customer),
                (StandardCommunity::new(174, 21100), PeerType::Peer),
                (StandardCommunity::new(174, 21101), PeerType::Customer),
                (StandardCommunity::new(174, 21200), PeerType::Peer),
                (StandardCommunity::new(174, 21201), PeerType::Customer),
                (StandardCommunity::new(174, 21300), PeerType::Peer),
                (StandardCommunity::new(174, 21301), PeerType::Customer),
                (StandardCommunity::new(174, 21400), PeerType::Peer),
                (StandardCommunity::new(174, 21401), PeerType::Customer),
                (StandardCommunity::new(174, 21500), PeerType::Peer),
                (StandardCommunity::new(174, 21501), PeerType::Customer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(174, 21000),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(174, 21001),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(174, 21100),
                    PeerLocation::EuropeMiddleEast,
                ),
                (
                    StandardCommunity::new(174, 21101),
                    PeerLocation::EuropeMiddleEast,
                ),
                (StandardCommunity::new(174, 21200), PeerLocation::AsiaPac),
                (StandardCommunity::new(174, 21201), PeerLocation::AsiaPac),
                (
                    StandardCommunity::new(174, 21300),
                    PeerLocation::SouthAmerica,
                ),
                (
                    StandardCommunity::new(174, 21301),
                    PeerLocation::SouthAmerica,
                ),
                (StandardCommunity::new(174, 21400), PeerLocation::AsiaPac),
                (StandardCommunity::new(174, 21401), PeerLocation::AsiaPac),
                (StandardCommunity::new(174, 21500), PeerLocation::Africa),
                (StandardCommunity::new(174, 21501), PeerLocation::Africa),
            ]),
        ),
    );

    /* Verizon */
    asn_mappings.insert(
        Asn::new_32bit(701),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(0, 201), PeerType::Customer),
                (StandardCommunity::new(0, 203), PeerType::Peer),
            ]),
            HashMap::new(),
        ),
    );

    /* Vodafone */
    let mut cm = CommMappings::new(HashMap::new(), HashMap::new());

    // Regional learned from customer communities
    for i in 11000..18999 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Customer);
    }

    // Regional learned from peer communities
    for i in 21000..28999 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Peer);
    }

    // Regional learned from upstream communities
    for i in 31000..38999 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Upstream);
    }

    // Leaned from upstream Arelion
    for i in 39970..39979 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Upstream);
    }

    // Learned in North America
    for i in 11000..11999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::NorthAmerica);
    }
    for i in 21000..21999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::NorthAmerica);
    }
    for i in 31000..31999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::NorthAmerica);
    }

    // Learned in Europe
    for i in 12000..12999 {
        cm.add_peer_location(
            StandardCommunity::new(1273, i),
            PeerLocation::EuropeMiddleEast,
        );
    }
    for i in 22000..22999 {
        cm.add_peer_location(
            StandardCommunity::new(1273, i),
            PeerLocation::EuropeMiddleEast,
        );
    }
    for i in 32000..32999 {
        cm.add_peer_location(
            StandardCommunity::new(1273, i),
            PeerLocation::EuropeMiddleEast,
        );
    }

    // Learned in Asia
    for i in 13000..13999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 23000..23999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 33000..33999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }

    // Learned in Australia
    for i in 14000..14999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 24000..24999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 34000..34999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }

    // Learned in South America
    for i in 15000..15999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::SouthAmerica);
    }
    for i in 25000..25999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::SouthAmerica);
    }
    for i in 35000..35999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::SouthAmerica);
    }

    // Learned in Africa
    for i in 16000..16999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Africa);
    }
    for i in 26000..26999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Africa);
    }
    for i in 36000..36999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Africa);
    }

    // Learned in Middle East
    for i in 17000..17999 {
        cm.add_peer_location(
            StandardCommunity::new(1273, i),
            PeerLocation::EuropeMiddleEast,
        );
    }
    for i in 27000..27999 {
        cm.add_peer_location(
            StandardCommunity::new(1273, i),
            PeerLocation::EuropeMiddleEast,
        );
    }
    for i in 37000..37999 {
        cm.add_peer_location(
            StandardCommunity::new(1273, i),
            PeerLocation::EuropeMiddleEast,
        );
    }

    // Learned in India
    for i in 18000..18999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 28000..28999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 38000..38999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }

    asn_mappings.insert(Asn::new_32bit(1273), cm);

    /* Arelion */
    asn_mappings.insert(
        Asn::new_32bit(1299),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(1299, 20000), PeerType::Peer),
                (StandardCommunity::new(1299, 25000), PeerType::Peer),
                (StandardCommunity::new(1299, 27000), PeerType::Peer),
                (StandardCommunity::new(1299, 30000), PeerType::Customer),
                (StandardCommunity::new(1299, 35000), PeerType::Customer),
                (StandardCommunity::new(1299, 37000), PeerType::Customer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(1299, 20000),
                    PeerLocation::EuropeMiddleEast,
                ),
                (
                    StandardCommunity::new(1299, 25000),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(1299, 27000), PeerLocation::AsiaPac),
                (
                    StandardCommunity::new(1299, 30000),
                    PeerLocation::EuropeMiddleEast,
                ),
                (
                    StandardCommunity::new(1299, 35000),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(1299, 37000), PeerLocation::AsiaPac),
            ]),
        ),
    );

    /* NTT */
    asn_mappings.insert(
        Asn::new_32bit(2914),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(2914, 410), PeerType::Customer),
                (StandardCommunity::new(2914, 420), PeerType::Peer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(2914, 3000),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(2914, 3075),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(2914, 3200),
                    PeerLocation::EuropeMiddleEast,
                ),
                (
                    StandardCommunity::new(2914, 3275),
                    PeerLocation::EuropeMiddleEast,
                ),
                (StandardCommunity::new(2914, 3400), PeerLocation::AsiaPac),
                (StandardCommunity::new(2914, 3475), PeerLocation::AsiaPac),
                (
                    StandardCommunity::new(2914, 3600),
                    PeerLocation::SouthAmerica,
                ),
                (
                    StandardCommunity::new(2914, 3675),
                    PeerLocation::SouthAmerica,
                ),
            ]),
        ),
    );

    /* GTT */
    let mut cm = CommMappings::new(
        HashMap::from([(StandardCommunity::new(3257, 4000), PeerType::Customer)]),
        HashMap::from([
            (
                StandardCommunity::new(3257, 50001),
                PeerLocation::EuropeMiddleEast,
            ),
            (
                StandardCommunity::new(3257, 50002),
                PeerLocation::NorthAmerica,
            ),
            (StandardCommunity::new(3257, 50003), PeerLocation::AsiaPac),
        ]),
    );

    // Regional learned from customer communities
    for i in 30000..39999 {
        cm.add_peer_type(StandardCommunity::new(3257, i), PeerType::Peer);
    }
    asn_mappings.insert(Asn::new_32bit(3257), cm);
}
