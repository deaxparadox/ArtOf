# Internetworking Baics

A basic *local area network (LAN)* that's connected using a *hub*, which is basically just an antiquated device that connects wires together. 

- Simple network like this would be coonsidred one **collision domain** on one **broadcast domain**.

![A simple network](assets/100-simple-network.png)

- Let understant this figure with this scenario: Bob wants to snd Sally a file, and to complete that goal in this kind of network, he'll simply broadcast that he's looking for her, which is basically just shouting out over the network. 

- Think of it like this: Bob walks out of his house and yells down a street called Chaos Court in order to contact Sally. This might work if Bob and Sally were the only ones  living there, but not so much if it’s crammed with homes and all the others living there  are always hollering up and down the street to their neighbors just like Bob. Nope, Chaos Court would absolutely live up to its name, with all those residents going off whenever they felt like it—and believe it or not, our networks actually still work this way to a  degree! So, given a choice, would you stay in Chaos Court, or would you pull up stakes and move on over to a nice new modern community called Broadway Lanes, which offers plenty of ame nities and room for your home plus future additions all on nice, wide streets that can easily handle all present and future traffic? If you chose the latter, good choice... so did Sally, and she now lives a much quieter life, getting letters  (packets) from Bob instead of a headache!.


And this is no joke; most of us think of growth as good—and it can be—but as many of us experience daily when commuting to work, school, etc., it can also mean your LAN’s traffic congestion can reach critical mass and grind to a complete halt! 

The Solution to this problem begins with breaking up a massive network into a number of smaller ones--something called ***network segmentation***. This concept is like planning a new community or modernizing an existing one. In a networking neighborhoood environment, all of this is carried out using devices like *routers*, *switches*, and *bridges*.

----------

This figure shows a network that's been segmented with a switch, making each network segment that connects to the switch it's own separate *collision domain*.

![Switch can break up collision domain](assets/101-switch-can-break-up-collision-domains.png)

- This is great, but this networkin is still one, single *broadcast domain*, meaning that we've really only decreased our screaming and yelling, not eliminated it.

- For example, if there’s some sort of vital announcement that everyone in our neighborhood needs to hear about, it will definitely still get loud! You can see that the hub used in Figure 1.2 just extended the one collision domain from the switch port. The result is that John received the data from Bob but, happily, Sally did not. This is good  because Bob intended to talk with John directly, and if he had needed to send a broadcast instead, everyone, including Sally, would have received it, possibly causing unnecessary  congestion.


Here's a list of some of things that commonly cause LAN traffic congestion:

- To many hosts in a collision or broadcast domain
- Broadcast storms
- Too much multicast traffic
- Low bandwidth
- Adding hubs for connectivity to the network
- A bunch of ARP broadcasts

----------

Hubs don’t segment a network; they just connect network segments. Basically, it’s an inexpensive way to connect a couple of PCs, and again, that’s great for home use and troubleshooting, but that’s about it!

Routers are used to connect  networks and route packets of data from one network to another. So never forget that by default, routers are basically employed to efficiently break up a *broadcast domain*—the set of all devices on a network segment, which are allowed to “hear” all broadcasts sent out on that specific segment.

![Routes create a internetwork](assets/103-router.png)

- Each host is connectet to its own collision domain because of the switch, and the router has created two broadcast domains.

- So now our Sally is happily living in peace in a completely different neighborhood,
no longer subjected to Bob’s incessant shouting! If Bob wants to talk with Sally, he has to


Routers provide connections to *wide area network (WAN)* services as well via a serial interface for WAN connections—specifically, a *V.35 physical interface on a
Cisco router*.


#### why breaking up a broadcast domain is so important

When a host or server sends a network broadcast, every device on the network must read
and process that broadcast—unless you have a router. When the router’s interface receives
this broadcast, it can respond by basically saying, “Thanks, but no thanks,” and discard
the broadcast without forwarding it on to other networks. Even though routers are known
for breaking up broadcast domains by default, it’s important to remember that they break
up collision domains as well.


There are two advantagse to using routes in your networks:

- They don't forward broadcasts by default
- They can fitler the network based on layer 3 (Network layer) information such as an IP address.

Here are four ways a router functions in your network:

- Packet Switching
- Packet filtering
- Internetwork communication
- Path selection


----------

- Routers are like layer 3 swtiches.
- Routers (layer 3 switches) uses logical addressing and provide an important capacity called **packet switching**.
- Routers also provide **packet filtering** via access lists, and when routers connect two or more networks together and use logical addressing (IP or IPv6), you then have an *internetwork*.
- Finally routers use a routing table, which is essentially a map of the internetwork, to make best path selections for getting data tot its proper destination and properly forward packets to remote networks.