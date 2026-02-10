# Who Pays Whom, and Where?

* Use RIPE RIS and RouteViews data to look at all routes with two back-to-back Tier 1 ASNs in the path.
* Check which network is attaching it's "learned from peer" community, and which is attaching it's "learned from customer" community.
  * And where are the routes being learned (based on any "learned in location" informational communities).
* Also look for routes with three Tier 1 ASNs in the path, this indicates customer->peer->peer or peer->peer->customer.

## Communities

I tried to gather a [list of known communities](Communities.md) for peer types and locations, for each Tier 1.

Of course, they split the world in different regions and peer types, there is no agreement on what format to use.

## Results

[View Result Table](https://htmlpreview.github.io/?https://github.com/jwbensley/wpw/blob/main/results/peerings.html)

Unfortunately RIS and RVs don't have great Tier 1 coverage. Also, most of the Tier 1 data visible via RIS and RV comes via customers, and some Tier 1s don't send informational communities unless explicitly requested, or only send them to peers, not customers.

* AS1273 Vodafone are not transit free, they use AS1299 Arelion in North America for connectivity.
* AS6461 Zayo are not transit free, they use AS2914 NTT in Asia for connectivity, AS3491 PCCW in Asia for connectivity, and AS6762 TISparkle in South America for connectivity.
* AS6939 Hurricane Electric are not transit free, they use AS1299 Arelion in Europe and North America for connectivity.
  * AS6939 also don't peer with AS174 Cogent, and despite taking IPv4 and IPv6 transit from AS1299, AS6939 don't have IPv6 connectivity with AS174.
  * This means that AS174 and AS6939 are perhaps the only two "Tier 1s" that don't actually have full table connectivity?

Set theory tells us that if AS1273 uses only one upstream, AS1299, but peers with all other Tier 1 networks, then they will only use 1299 to reach 1299's customer cone, so despite paying, 1273 are kind of transit free.

AS6461 has to pay multiple networks for connectivity, _in the same region_, so they are not transit free. This is complicated by the fact they don't pay in all regions. Inter-region connectivity is expensive, and each Tier 1 network is strong in certain regions and weaker in others. Paying TISparkle in South America makes sense because they are strong there, NTT and PCCW are strong in Asia, Zayo themselves are strong in the USA. Although, it's not shown by the RIS and RouteViews data, 6461 is [peering with](https://bgp.he.net/super-lg/#5.133.162.0/24?tob=none&mt=include&ma=6461&mn=2620:135:5002:30::131,collector9.bgp.he.net&els=exact) 6830, and 6461 is [peering with](https://bgp.he.net/super-lg/#23.146.80.0/24?tob=none&mn=91.206.52.115,rrc20&els=exact) 6939, but not in all regions it seems.

There is minimal BGP routing information available for AS701 and AS7018 via public route collectors. AS701 has a [very limited](Communities.md#as701) set of informational communities, [as does](Communities.md#as7018) AS7018. It has been shown in the past that these two networks heavy rely on other Tier 1s to reach the rest of the DFZ, [for IPv4](https://tier1-analysis.53bits.co.uk/part3/2025/plots/v4_shorter_t1.html), and [for IPv6](https://tier1-analysis.53bits.co.uk/part3/2025/plots/v6_shorter_t1.html).

## Usage

Setup:

```shell
cargo build -r
python3 -m venv .venv && source .venv/bin/activate
python3 -m pip install -r requirements.txt
```

Download all RIS and RouteViews RIB dumbs for a specific day, at midnight, and parse them using 15 threads.

Then plot the discovered peerings.

```shell
./target/release/who-pays-whom -t 15 download -p /opt/mrts/20260204/ -y 20260204
./plot.py
```

Pull extra MRTs from the BGPRoutes.io public MRT archive, and reparse everything to see if we get anymore insights...

```shell
./download_bgproutes.io.sh /opt/mrts/20260204/
./target/release/who-pays-whom -t 15 files -f /opt/mrts/20260204/*
./plot.py
```

Nope :grin:
