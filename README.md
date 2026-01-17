# Who Pays Whom, and Where?

Who is attaching "learned from peer" communities, and who is attaching "learned from customer" communities? And where are they routes being learned (based on "learned in location" communities).


https://www.cogentco.com/files/docs/customer_service/guide/global_cogent_customer_user_guide.pdf

174:21001 Route is NA internal or customer route.
174:21101 Route is an EU internal or customer route.
174:21201 Route is an AP internal or customer route.
174:21301 Route is a SA internal or customer route.
174:21401 Route is an AU internal or customer route.
174:21501 Route is an AF internal or customer route.
174:21000 Route is learned from NA (North America) non-customer.
174:21100 Route is learned from EU (Europe) non-customer.
174:21200 Route is learned from AP (Asia Pacific) non-customer.
174:21300 Route is learned from SA (South America) non-customer.
174:21400 Route is learned from AU (Australia) non-customer.
174:21500 Route is learned from AF (Africa) non-customer.




https://onestep.net/documents/as701-bordergateproto.pdf
https://www.verizon.com/business/why-verizon/looking-glass/

0:201 - Leanred from customer
0:203 - Learned from peer




https://www.vodafone.com/business/privacy/customer-taggable-community-attribute-values
https://portal.vodafone.com/web/lookingglass

  	1273:11xxx 	customer route (North America)
  	1273:12xxx 	customer route (Europe)
  	1273:13xxx 	customer route (Asia)
  	1273:14xxx 	customer route (Australia)
  	1273:15xxx 	customer route (South America)
    1273:16xxx 	customer route (Africa)        -> Unconfirmed
    1273:17xxx 	customer route (Middle East)   -> Unconfirmed
    1273:18xxx 	customer route (India)         -> Unconfirmed
  	1273:21xxx 	peer route (North America)
  	1273:22xxx 	peer route (Europe)
  	1273:23xxx 	peer route (Asia)
  	1273:24xxx 	peer route (Australia)
  	1273:25xxx 	peer route (South America)
    1273:26xxx 	peer route (Africa)            -> Unconfirmed
    1273:27xxx 	peer route (Middle East)       -> Unconfirmed
    1273:28xxx 	peer route (India)             -> Unconfirmed
  	1273:31xxx 	upstream route (North America)
  	1273:32xxx 	upstream route (Europe)
  	1273:33xxx 	upstream route (Asia)
  	1273:34xxx 	upstream route (Australia)
  	1273:35xxx 	upstream route (South America)
    1273:36xxx 	upstream route (Africa)        -> Unconfirmed
    1273:37xxx 	upstream route (Middle East)   -> Unconfirmed
    1273:38xxx 	upstream route (India)         -> Unconfirmed

1273:3997x - Upstream Telia (AS1299)




https://www.arelion.com/our-network/bgp-routing/bgp-communities

Community numbering uses the following structure:
    1299:xyzzz
    Where: 
    x is BGP Neighbour type; 2 for Peers or 3 for Customers
    y is Region; 0 for Europe, 5 for North America or 7 for Asia & Pacific
    zzz is City; see below

1299:20000 EU Peers
1299:25000 North American Peers
1299:27000 Asia & Pacific Peers
1299:30000 EU Customers
1299:35000 North American Customers
1299:37000 Asia & Pacific Customers


https://www.gin.ntt.net/support-center/policies-procedures/routing/

2914:410 	NTT DATA and customer routes
2914:420 	Peer routes

world regional origins (2914:3—)
2914:3000 	North America
2914:3075 	North America regional customer
2914:3200 	Europe
2914:3275 	Europe regional customer
2914:3400 	Asia
2914:3475 	Asia regional customer
2914:3600 	South America
2914:3675 	South America regional customer


https://www.gtt.net/services/managed-networking/internet/ip-transit/bgp-communities/

3257:4000 	tags a GTT customer route. 
3257:50001 	route originated in Europe
3257:50002 	route originated in the US and Canada
3257:50003 	route originated in Asia
3257:30000-39999 	Auto-incremental number of private interconnects 


https://wholesale.telekom.com/fileadmin/user_upload/documents/global/downloads/BGP_Signaling_for_AS3320.pdf

3320:2010 Imported in Europe tag.origin.region.eu
3320:2020 Imported in North America tag.origin.region.na
3320:2030 Imported in Pacific Rim tag.origin.region.pacrim
3320:9010 Imported from customer tag.origin.type.customer
3320:9020 Imported from peer tag.origin.type.peer



$ whois -h rr.level3.net -s LEVEL3 AS3356

3356:123  - Customer route
3356:666  - Peer route
3356:2    - Europe
3356:3    - North America
3356:4    - APAC
3356:5    - LATAM




$ whois -h whois.radb.net AS3491

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




https://wholesale.orange.com/portail/resources/other/Orange_BGP_Best_Practices_for_IPT_Customers.pdf

$ whois -h whois.ripe.net AS5511

5511:666 - Peer route
5511:999 - Customer route
remarks:        5511:30100 Caribbean
remarks:        5511:560 Caribbean Customers (Deprecated)
remarks:        5511:700 Caribbean Peer & contents (Deprecated)
remarks:        5511:30106 Eastern Africa
remarks:        5511:640 Eastern Africa Customers (Deprecated)
remarks:        5511:730 Eastern Africa Peer & contents (Deprecated)
remarks:        5511:30121 Eastern Asia
remarks:        5511:600 Eastern Asia Customers (Deprecated)
remarks:        5511:720 Eastern Asia Peer & contents (Deprecated)
remarks:        5511:30139 Eastern Europe
remarks:        5511:500 Eastern Europe Customers (Deprecated)
remarks:        5511:710 Eastern Europe Peer & contents (Deprecated)
remarks:        5511:30173 Middle Africa
remarks:        5511:640 Middle Africa Customers (Deprecated)
remarks:        5511:730 Middle Africa Peer & contents (Deprecated)
remarks:        5511:30184 Northern Africa
remarks:        5511:640 Northern Africa Customers (Deprecated)
remarks:        5511:730 Northern Africa Peer & contents (Deprecated)
remarks:        5511:30194 Northern America (East)
remarks:        5511:540 Northern America (East) Customers (Deprecated)
remarks:        5511:700 Northern America (East) Peer & contents (Deprecated)
remarks:        5511:30218 Northern America (West)
remarks:        5511:540 Northern America (West) Customers (Deprecated)
remarks:        5511:700 Northern America (West) Peer & contents (Deprecated)
remarks:        5511:30228 Northern Europe
remarks:        5511:500 Northern Europe Customers (Deprecated)
remarks:        5511:710 Northern Europe Peer & contents (Deprecated)
remarks:        5511:30237 Oceania
remarks:        5511:680 Oceania Customers (Deprecated)
remarks:        5511:680 Oceania Peer & contents (Deprecated)
remarks:        5511:30241 South-eastern Asia
remarks:        5511:600 South-eastern Asia Customers (Deprecated)
remarks:        5511:720 South-eastern Asia Peer & contents (Deprecated)
remarks:        5511:30251 Southern Africa
remarks:        5511:640 Southern Africa Customers (Deprecated)
remarks:        5511:730 Southern Africa Peer & contents (Deprecated)
remarks:        5511:30541 Southern America
remarks:        5511:590 Southern America Customers (Deprecated)
remarks:        5511:700 Southern America Peer & contents (Deprecated)
remarks:        5511:30257 Southern Europe
remarks:        5511:500 Southern Europe Customers (Deprecated)
remarks:        5511:710 Southern Europe Peer & contents (Deprecated)
remarks:        5511:30343 Western Africa
remarks:        5511:640 Western Africa Customers (Deprecated)
remarks:        5511:730 Western Africa Peer & contents (Deprecated)
remarks:        5511:30416 Western Asia
remarks:        5511:650 Western Asia Customers (Deprecated)
remarks:        5511:720 Western Asia Peer & contents (Deprecated)
remarks:        5511:30428 Western Europe
remarks:        5511:500 Western Europe Customers (Deprecated)
remarks:        5511:710 Western Europe Peer & contents (Deprecated)




https://www.scribd.com/document/399871041/TATA-AS6453-BGP-Communities

6453:1000 North America
6453:2000 Europe
6453:3000 Asia Pacific
6453:4000 Middle East and Afria
6453:6000 India
6453:50 - Customer Route
6453:86 - Peer Route




No verification of these - wait on email response
6461:1114   ??? Possible EU Peer identifier
6461:2101   Customer in USA
6461:2601   Peer in USA
6461:5994	Prefixes learned via a peering session in APAC		
6461:5995	Prefixes learned across a public peering exchange		
6461:5996	Prefixes learned via a peering session in the EU		
6461:5997	learned from a peer		
6461:5998	learned from a customer		
6461:5999	learned from an internal resource	




https://www.tisparkle.com/BGP-Community-support-for-AS6762-Customers
6762:30 	Route learnt in Europe
6762:31 	Route learnt in North-America
6762:32 	Route learnt in South-America
6762:33 	Route learnt in Asia
6762:34 	Route learnt in Africa
6762:40 	Route learnt from a customer



Unconfirmed - https://lg.aorta.net/app
6830:13000  Customer prefix
6830:16000 	Learned from public peers (via an IXP)
6830:17000 	Learned from private peers
6830,23001



Unconfirmed
6939:1000 	Customer Route
6939:9001 	Learned in North America
6939:9002 	Learned in EU
6939:9003 	Learned in APAC
6939:9004 	Learned in Africa
6939:9005 	Learned in South America
6939:9006 	Learned in Oceania
6939:9007 	Learned in Middle East



Unconfirmed
7018:2000 	Learned from Customer
7018:5000 	Learned from Peer




Unconfirmed - https://telxius.com/en/looking-glass-service/
   	12956:4001 	Learned in Europe
  	12956:4002 	Learned in South/Central America
  	12956:4003 	Learned in North America
  	12956:4004 	Learned in Asia
  	12956:4005 	Learned in Africa
