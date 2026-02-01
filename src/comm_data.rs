use crate::comm_mappings::community_mappings::CommMappings;
use crate::mrt_asn::asn::MrtAsn;
use crate::mrt_communities::standard_communities::StandardCommunity;
use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
use std::collections::HashMap;

pub fn insert_comm_mapping(asn_mappings: &mut HashMap<MrtAsn, CommMappings>) {
    /* Cogent */
    asn_mappings.insert(
        MrtAsn::from_u32(174),
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
                (StandardCommunity::new(174, 21100), PeerLocation::Europe),
                (StandardCommunity::new(174, 21101), PeerLocation::Europe),
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
        MrtAsn::from_u32(701),
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
    for i in 11000..=18999 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Customer);
    }

    // Regional learned from peer communities
    for i in 21000..=28999 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Peer);
    }

    // Regional learned from upstream communities
    for i in 31000..=38999 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Upstream);
    }

    // Leaned from upstream Arelion
    for i in 39970..=39979 {
        cm.add_peer_type(StandardCommunity::new(1273, i), PeerType::Upstream);
    }

    // Learned in North America
    for i in 11000..=11999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::NorthAmerica);
    }
    for i in 21000..=21999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::NorthAmerica);
    }
    for i in 31000..=31999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::NorthAmerica);
    }

    // Learned in Europe
    for i in 12000..=12999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Europe);
    }
    for i in 22000..=22999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Europe);
    }
    for i in 32000..=32999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Europe);
    }

    // Learned in Asia
    for i in 13000..=13999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 23000..=23999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 33000..=33999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }

    // Learned in Australia
    for i in 14000..=14999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 24000..=24999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 34000..=34999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }

    // Learned in South America
    for i in 15000..=15999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::SouthAmerica);
    }
    for i in 25000..=25999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::SouthAmerica);
    }
    for i in 35000..=35999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::SouthAmerica);
    }

    // Learned in Africa
    for i in 16000..=16999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Africa);
    }
    for i in 26000..=26999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Africa);
    }
    for i in 36000..=36999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Africa);
    }

    // Learned in Middle East
    for i in 17000..=17999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Europe);
    }
    for i in 27000..=27999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Europe);
    }
    for i in 37000..=37999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::Europe);
    }

    // Learned in India
    for i in 18000..=18999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 28000..=28999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }
    for i in 38000..=38999 {
        cm.add_peer_location(StandardCommunity::new(1273, i), PeerLocation::AsiaPac);
    }

    asn_mappings.insert(MrtAsn::from_u32(1273), cm);

    /* Arelion */
    asn_mappings.insert(
        MrtAsn::from_u32(1299),
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
                (StandardCommunity::new(1299, 20000), PeerLocation::Europe),
                (
                    StandardCommunity::new(1299, 25000),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(1299, 27000), PeerLocation::AsiaPac),
                (StandardCommunity::new(1299, 30000), PeerLocation::Europe),
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
        MrtAsn::from_u32(2914),
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
                (StandardCommunity::new(2914, 3200), PeerLocation::Europe),
                (StandardCommunity::new(2914, 3275), PeerLocation::Europe),
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
            (StandardCommunity::new(3257, 50001), PeerLocation::Europe),
            (
                StandardCommunity::new(3257, 50002),
                PeerLocation::NorthAmerica,
            ),
            (StandardCommunity::new(3257, 50003), PeerLocation::AsiaPac),
        ]),
    );

    // Regional learned from peer communities
    for i in 30000..=39999 {
        cm.add_peer_type(StandardCommunity::new(3257, i), PeerType::Peer);
    }
    asn_mappings.insert(MrtAsn::from_u32(3257), cm);

    /* DTAG */
    asn_mappings.insert(
        MrtAsn::from_u32(3320),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(3320, 9010), PeerType::Customer),
                (StandardCommunity::new(3320, 9020), PeerType::Peer),
            ]),
            HashMap::from([
                (StandardCommunity::new(3320, 2010), PeerLocation::Europe),
                (
                    StandardCommunity::new(3320, 2020),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(3320, 2030), PeerLocation::AsiaPac),
            ]),
        ),
    );

    /* Lumen */
    asn_mappings.insert(
        MrtAsn::from_u32(3356),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(3356, 123), PeerType::Customer),
                (StandardCommunity::new(3356, 666), PeerType::Peer),
            ]),
            HashMap::from([
                (StandardCommunity::new(3356, 2), PeerLocation::Europe),
                (StandardCommunity::new(3356, 3), PeerLocation::NorthAmerica),
                (StandardCommunity::new(3356, 4), PeerLocation::AsiaPac),
                (StandardCommunity::new(3356, 5), PeerLocation::SouthAmerica),
            ]),
        ),
    );

    /* PCCW */
    asn_mappings.insert(
        MrtAsn::from_u32(3491),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(3491, 100), PeerType::Customer),
                (StandardCommunity::new(3491, 200), PeerType::Customer),
                (StandardCommunity::new(3491, 300), PeerType::Customer),
                (StandardCommunity::new(3491, 400), PeerType::Customer),
                (StandardCommunity::new(3491, 500), PeerType::Customer),
                (StandardCommunity::new(3491, 700), PeerType::Customer),
                (StandardCommunity::new(3491, 1000), PeerType::Peer),
                (StandardCommunity::new(3491, 2000), PeerType::Peer),
                (StandardCommunity::new(3491, 3000), PeerType::Peer),
                (StandardCommunity::new(3491, 4000), PeerType::Peer),
                (StandardCommunity::new(3491, 5000), PeerType::Peer),
                (StandardCommunity::new(3491, 7000), PeerType::Peer),
                (StandardCommunity::new(3491, 9001), PeerType::Customer),
                (StandardCommunity::new(3491, 9002), PeerType::Peer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(3491, 100),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(3491, 200),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(3491, 300), PeerLocation::Europe),
                (StandardCommunity::new(3491, 400), PeerLocation::AsiaPac),
                (StandardCommunity::new(3491, 500), PeerLocation::Africa),
                (StandardCommunity::new(3491, 700), PeerLocation::AsiaPac),
                (
                    StandardCommunity::new(3491, 1000),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(3491, 2000),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(3491, 3000), PeerLocation::Europe),
                (StandardCommunity::new(3491, 4000), PeerLocation::AsiaPac),
                (StandardCommunity::new(3491, 5000), PeerLocation::Africa),
                (StandardCommunity::new(3491, 7000), PeerLocation::AsiaPac),
            ]),
        ),
    );

    /* Orange */
    asn_mappings.insert(
        MrtAsn::from_u32(5511),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(5511, 666), PeerType::Peer),
                // (StandardCommunity::new(5511, 680), PeerType::Peer),
                // (StandardCommunity::new(5511, 700), PeerType::Peer),
                // (StandardCommunity::new(5511, 710), PeerType::Peer),
                // (StandardCommunity::new(5511, 720), PeerType::Peer),
                // (StandardCommunity::new(5511, 730), PeerType::Peer),
                // (StandardCommunity::new(5511, 500), PeerType::Customer),
                // (StandardCommunity::new(5511, 540), PeerType::Customer),
                // (StandardCommunity::new(5511, 590), PeerType::Customer),
                // (StandardCommunity::new(5511, 560), PeerType::Customer),
                // (StandardCommunity::new(5511, 600), PeerType::Customer),
                // (StandardCommunity::new(5511, 640), PeerType::Customer),
                // (StandardCommunity::new(5511, 650), PeerType::Customer),
                // (StandardCommunity::new(5511, 680), PeerType::Customer),
                (StandardCommunity::new(5511, 999), PeerType::Customer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(5511, 30100),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(5511, 560),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(5511, 700),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(5511, 30106), PeerLocation::Africa),
                (StandardCommunity::new(5511, 640), PeerLocation::Africa),
                (StandardCommunity::new(5511, 730), PeerLocation::Africa),
                (StandardCommunity::new(5511, 30121), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 600), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 720), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 30139), PeerLocation::Europe),
                (StandardCommunity::new(5511, 500), PeerLocation::Europe),
                (StandardCommunity::new(5511, 710), PeerLocation::Europe),
                (StandardCommunity::new(5511, 30173), PeerLocation::Africa),
                (StandardCommunity::new(5511, 30184), PeerLocation::Africa),
                (
                    StandardCommunity::new(5511, 30194),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(5511, 540),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(5511, 30218),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(5511, 30228), PeerLocation::Europe),
                (StandardCommunity::new(5511, 30237), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 680), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 30241), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 30251), PeerLocation::Africa),
                (
                    StandardCommunity::new(5511, 30541),
                    PeerLocation::SouthAmerica,
                ),
                (
                    StandardCommunity::new(5511, 590),
                    PeerLocation::SouthAmerica,
                ),
                (StandardCommunity::new(5511, 30257), PeerLocation::Europe),
                (StandardCommunity::new(5511, 30343), PeerLocation::Africa),
                (StandardCommunity::new(5511, 30416), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 650), PeerLocation::AsiaPac),
                (StandardCommunity::new(5511, 30428), PeerLocation::Europe),
            ]),
        ),
    );

    /* TATA */
    asn_mappings.insert(
        MrtAsn::from_u32(6453),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(6453, 50), PeerType::Customer),
                (StandardCommunity::new(6453, 86), PeerType::Peer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(6453, 1000),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(6453, 2000), PeerLocation::Europe),
                (StandardCommunity::new(6453, 3000), PeerLocation::AsiaPac),
                (StandardCommunity::new(6453, 4000), PeerLocation::Africa),
                (StandardCommunity::new(6453, 6000), PeerLocation::AsiaPac),
            ]),
        ),
    );

    /* Zayo */
    asn_mappings.insert(
        MrtAsn::from_u32(6461),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(6461, 2101), PeerType::Customer),
                (StandardCommunity::new(6461, 2601), PeerType::Peer),
                (StandardCommunity::new(6461, 5994), PeerType::Peer),
                (StandardCommunity::new(6461, 5995), PeerType::Peer),
                (StandardCommunity::new(6461, 5996), PeerType::Peer),
                (StandardCommunity::new(6461, 5997), PeerType::Peer),
                (StandardCommunity::new(6461, 5998), PeerType::Customer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(6461, 2101),
                    PeerLocation::NorthAmerica,
                ),
                (
                    StandardCommunity::new(6461, 2601),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(6461, 5994), PeerLocation::AsiaPac),
                (StandardCommunity::new(6461, 5996), PeerLocation::Europe),
            ]),
        ),
    );

    /* TISparkle */
    asn_mappings.insert(
        MrtAsn::from_u32(6762),
        CommMappings::new(
            HashMap::from([(StandardCommunity::new(6762, 40), PeerType::Customer)]),
            HashMap::from([
                (StandardCommunity::new(6762, 30), PeerLocation::Europe),
                (StandardCommunity::new(6762, 31), PeerLocation::NorthAmerica),
                (StandardCommunity::new(6762, 32), PeerLocation::SouthAmerica),
                (StandardCommunity::new(6762, 33), PeerLocation::AsiaPac),
                (StandardCommunity::new(6762, 34), PeerLocation::Africa),
            ]),
        ),
    );

    /* Libery Global */
    asn_mappings.insert(
        MrtAsn::from_u32(6830),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(6830, 13000), PeerType::Customer),
                (StandardCommunity::new(6830, 16000), PeerType::Peer),
                (StandardCommunity::new(6830, 17000), PeerType::Peer),
            ]),
            HashMap::new(),
        ),
    );

    /* Hurricane Electric */
    asn_mappings.insert(
        MrtAsn::from_u32(6939),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(6939, 1000), PeerType::Customer),
                (StandardCommunity::new(6939, 16000), PeerType::Peer),
                (StandardCommunity::new(6939, 17000), PeerType::Peer),
            ]),
            HashMap::from([
                (
                    StandardCommunity::new(6939, 9001),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(6939, 9002), PeerLocation::Europe),
                (StandardCommunity::new(6939, 9003), PeerLocation::AsiaPac),
                (StandardCommunity::new(6939, 9004), PeerLocation::Africa),
                (
                    StandardCommunity::new(6939, 9005),
                    PeerLocation::SouthAmerica,
                ),
                (StandardCommunity::new(6939, 9006), PeerLocation::AsiaPac),
                (StandardCommunity::new(6939, 9007), PeerLocation::MiddleEast),
            ]),
        ),
    );

    /* AT&T */
    asn_mappings.insert(
        MrtAsn::from_u32(7018),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(7018, 2000), PeerType::Customer),
                (StandardCommunity::new(7018, 5000), PeerType::Peer),
            ]),
            HashMap::new(),
        ),
    );

    /* Telxius */
    asn_mappings.insert(
        MrtAsn::from_u32(12956),
        CommMappings::new(
            HashMap::from([
                (StandardCommunity::new(12956, 123), PeerType::Customer),
                (StandardCommunity::new(12956, 321), PeerType::Peer),
                (StandardCommunity::new(12956, 322), PeerType::PaidPeer),
            ]),
            HashMap::from([
                (StandardCommunity::new(12956, 4001), PeerLocation::Europe),
                (
                    StandardCommunity::new(12956, 4002),
                    PeerLocation::SouthAmerica,
                ),
                (
                    StandardCommunity::new(12956, 4003),
                    PeerLocation::NorthAmerica,
                ),
                (StandardCommunity::new(12956, 4004), PeerLocation::AsiaPac),
                (StandardCommunity::new(12956, 4005), PeerLocation::Africa),
            ]),
        ),
    );
}
