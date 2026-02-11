# BGP Communities

## AS174

Sources:

* <https://www.cogentco.com/files/docs/customer_service/guide/global_cogent_customer_user_guide.pdf>

Communities:

```text
174:21000 Route is learned from NA (North America) non-customer.
174:21001 Route is NA internal or customer route.
174:21100 Route is learned from EU (Europe) non-customer.
174:21101 Route is an EU internal or customer route.
174:21200 Route is learned from AP (Asia Pacific) non-customer.
174:21201 Route is an AP internal or customer route.
174:21300 Route is learned from SA (South America) non-customer.
174:21301 Route is a SA internal or customer route.
174:21400 Route is learned from AU (Australia) non-customer.
174:21401 Route is an AU internal or customer route.
174:21500 Route is learned from AF (Africa) non-customer.
174:21501 Route is an AF internal or customer route.
```

## AS701

Sources:

* <https://onestep.net/documents/as701-bordergateproto.pdf>
* <https://www.verizon.com/business/why-verizon/looking-glass/>

Communities:

```text
0:201 Learned from customer
0:203 Learned from peer
```

## AS1273

Sources:

* <https://www.vodafone.com/business/privacy/customer-taggable-community-attribute-values>
* <https://portal.vodafone.com/web/lookingglass>
* <https://bgp.tools/communities/1273>
* Still waiting on response to email with Vodafone

Communities:

```text
1273:11xxx customer route (North America)
1273:12xxx customer route (Europe)
1273:13xxx customer route (Asia)
1273:14xxx customer route (Australia)
1273:15xxx customer route (South America)
1273:16xxx customer route (Africa)        -> Unconfirmed
1273:17xxx customer route (Middle East)   -> Unconfirmed
1273:18xxx customer route (India)         -> Unconfirmed
1273:21xxx peer route (North America)
1273:22xxx peer route (Europe)
1273:23xxx peer route (Asia)
1273:24xxx peer route (Australia)
1273:25xxx peer route (South America)
1273:26xxx peer route (Africa)            -> Unconfirmed
1273:27xxx peer route (Middle East)       -> Unconfirmed
1273:28xxx peer route (India)             -> Unconfirmed
1273:31xxx upstream route (North America)
1273:32xxx upstream route (Europe)
1273:33xxx upstream route (Asia)
1273:34xxx upstream route (Australia)
1273:35xxx upstream route (South America)
1273:36xxx upstream route (Africa)        -> Unconfirmed
1273:37xxx upstream route (Middle East)   -> Unconfirmed
1273:38xxx upstream route (India)         -> Unconfirmed
1273:3997x Upstream Telia (AS1299)
```

## AS1299

Sources:

* <https://www.arelion.com/our-network/bgp-routing/bgp-communities>

Communities:

```text
Community numbering uses the following structure:
1299:xyzzz
Where: 
x is BGP Neighbour type; 2 for Peers or 3 for Customers
y is Region; 0 for Europe, 5 for North America or 7 for Asia & Pacific
zzz is City

1299:20000 EU Peers
1299:25000 North American Peers
1299:27000 Asia & Pacific Peers
1299:30000 EU Customers
1299:35000 North American Customers
1299:37000 Asia & Pacific Customers
```

## AS2914

Sources:

* <https://www.gin.ntt.net/support-center/policies-procedures/routing/>

Communities:

```text
2914:410 NTT DATA and customer routes
2914:420 Peer routes

world regional origins (2914:3*)
2914:3000 North America
2914:3075 North America regional customer
2914:3200 Europe
2914:3275 Europe regional customer
2914:3400 Asia
2914:3475 Asia regional customer
2914:3600 South America
2914:3675 South America regional customer
```

## AS3257

Sources:

* <https://www.gtt.net/services/managed-networking/internet/ip-transit/bgp-communities/>

Communities:

```text
3257:4000 GTT customer route. 
3257:30000-39999 Auto-incremental number of private interconnects 
3257:50001 Route originated in Europe
3257:50002 Route originated in the US and Canada
3257:50003 Route originated in Asia
```

## AS3320

Sources:

* <https://wholesale.telekom.com/fileadmin/user_upload/documents/global/downloads/BGP_Signaling_for_AS3320.pdf>

Communities:

```text
3320:2010 Imported in Europe tag.origin.region.eu
3320:2020 Imported in North America tag.origin.region.na
3320:2030 Imported in Pacific Rim tag.origin.region.pacrim
3320:9010 Imported from customer tag.origin.type.customer
3320:9020 Imported from peer tag.origin.type.peer
```

## AS3356

Sources:

* `$ whois -h rr.level3.net -s LEVEL3 AS3356`

Communities:

```text
3356:123 Customer route
3356:666 Peer route

3356:2 Europe
3356:3 North America
3356:4 APAC
3356:5 LATAM
```

## AS3491

Sources:

* `$ whois -h whois.radb.net AS3491`

Communities:

```text
remarks:        Region                          Customers   Peers
remarks:        -----------------------------------------------------
remarks:        North America (East Coast)      3491:100    3491:1000
remarks:        North America (West Coast)      3491:200    3491:2000
remarks:        Europe                          3491:300    3491:3000
remarks:        Asia                            3491:400    3491:4000
remarks:        Africa                          3491:500    3491:5000
remarks:        Australia                       3491:700    3491:7000

remarks:        Description               Community
remarks:        -----------------------------------
remarks:        3491:9001                 Customer
remarks:        3491:9002                 Peer
remarks:        3491:9003                 Self Originated
```

## AS5511

Sources:

* <https://wholesale.orange.com/portail/resources/other/Orange_BGP_Best_Practices_for_IPT_Customers.pdf>
* `$ whois -h whois.ripe.net AS5511`

Communities:

```text
5511:666 - Peer route
5511:680 Oceania Peer & contents (Deprecated)
5511:700 Northern America (East) Peer & contents (Deprecated)
5511:700 Caribbean Peer & contents (Deprecated)
5511:700 Northern America (West) Peer & contents (Deprecated)
5511:700 Southern America Peer & contents (Deprecated)
5511:710 Northern Europe Peer & contents (Deprecated)
5511:710 Eastern Europe Peer & contents (Deprecated)
5511:710 Southern Europe Peer & contents (Deprecated)
5511:710 Western Europe Peer & contents (Deprecated)
5511:720 Eastern Asia Peer & contents (Deprecated)
5511:720 South-eastern Asia Peer & contents (Deprecated)
5511:720 Western Asia Peer & contents (Deprecated)
5511:730 Eastern Africa Peer & contents (Deprecated)
5511:730 Middle Africa Peer & contents (Deprecated)
5511:730 Northern Africa Peer & contents (Deprecated)
5511:730 Southern Africa Peer & contents (Deprecated)
5511:730 Western Africa Peer & contents (Deprecated)

5511:999 - Customer route
5511:500 Southern Europe Customers (Deprecated)
5511:500 Northern Europe Customers (Deprecated)
5511:500 Eastern Europe Customers (Deprecated)
5511:500 Western Europe Customers (Deprecated)
5511:540 Northern America (West) Customers (Deprecated)
5511:540 Northern America (East) Customers (Deprecated)
5511:590 Southern America Customers (Deprecated)
5511:560 Caribbean Customers (Deprecated)
5511:600 South-eastern Asia Customers (Deprecated)
5511:600 Eastern Asia Customers (Deprecated)
5511:640 Middle Africa Customers (Deprecated)
5511:640 Southern Africa Customers (Deprecated)
5511:640 Western Africa Customers (Deprecated)
5511:640 Eastern Africa Customers (Deprecated)
5511:640 Northern Africa Customers (Deprecated)
5511:650 Western Asia Customers (Deprecated)
5511:680 Oceania Customers (Deprecated)
 
Origins:
5511:30100 Caribbean
5511:30106 Eastern Africa
5511:30121 Eastern Asia
5511:30139 Eastern Europe
5511:30173 Middle Africa
5511:30184 Northern Africa
5511:30194 Northern America (East)
5511:30218 Northern America (West)
5511:30228 Northern Europe
5511:30237 Oceania
5511:30241 South-eastern Asia
5511:30251 Southern Africa
5511:30541 Southern America
5511:30257 Southern Europe
5511:30343 Western Africa
5511:30416 Western Asia
5511:30428 Western Europe
```

## AS6453

Sources:

* <https://www.scribd.com/document/399871041/TATA-AS6453-BGP-Communities>
* <https://bgp.tools/communities/6453>

Communities:

```text
6453:1000 North America
6453:2000 Europe
6453:3000 Asia Pacific
6453:4000 Middle East and Africa
6453:6000 India
6453:50 Customer Route
6453:86 Peer Route
```

## AS6461

Sources:

* <https://bgp.tools/communities/6461>
* Still waiting on email response from Zayo

Communities:

```text
6461:1114 ??? Possible EU Peer identifier
6461:2101 Customer in USA
6461:2601 Peer in USA
6461:5994 Prefixes learned via a peering session in APAC
6461:5995 Prefixes learned across a public peering exchange
6461:5996 Prefixes learned via a peering session in the EU
6461:5997 learned from a peer
6461:5998 learned from a customer
6461:5999 learned from an internal resource  
```

## AS6762

Sources:

* <https://www.tisparkle.com/BGP-Community-support-for-AS6762-Customers>

Communities:

```text
6762:30 Route learnt in Europe
6762:31 Route learnt in North-America
6762:32 Route learnt in South-America
6762:33 Route learnt in Asia
6762:34 Route learnt in Africa
6762:40 Route learnt from a customer
```

## AS6830

Sources:

* <https://lg.aorta.net/app>
* <https://bgp.tools/communities/6830>
* Confirmed via DM

Communities

```text
6830:13000 Customer prefix
6830:16000 Learned from public peers (via an IXP)
6830:17000 Learned from private peers
6830:23001 Propagated through a route reflector

6830:343xx Learned in Austria
6830:332xx Learned in Belgium
6830:341xx Learned in Switzerland
6830:349xx Learned in Germany
6830:333xx Learned in France
6830:335xx Learned in Ireland
6830:339xx Learned in Italy
6830:331xx Learned in the Netherlands
6830:348xx Learned in Poland
6830:351xx Learned in Slovakia
6830:344xx Learned in United Kingdom
6830:353xx Learned in United States
```

## AS6939

Sources:

* <https://bgp.tools/communities/6939>
* Via email with HE

Communities:

```text
6939:1000 Customer Route
6939:9001 Learned in North America
6939:9002 Learned in EU
6939:9003 Learned in APAC
6939:9004 Learned in Africa
6939:9005 Learned in South America
6939:9006 Learned in Oceania
6939:9007 Learned in Middle East
6939:7XXX Router ID  
6939:8XXX Country - ISO 3166 Country Code https://en.wikipedia.org/wiki/ISO_3166-1#Current_codes  
6939:9XXX Region ID (1=North America, 2=EU, 3=APAC, 4=Africa, 5=South America, 6=Oceania, 7=Middle East)
```

## AS7018

Sources:

* <https://bgp.tools/communities/7018>

Communities:

```text
7018:2000 Learned from Customer
7018:5000 Learned from Peer
```

## AS12956

* <https://telxius.com/en/looking-glass-service/>
* Via email with Telxius

Communities:

```text
12956:123 Learned from customer
12956:321 Learned from peer
12956:322 Learned from paid peer
12956:4001 Learned in Europe
12956:4002 Learned in South/Central America
12956:4003 Learned in North America
12956:4004 Learned in Asia
12956:4005 Learned in Africa
```
