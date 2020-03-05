# Venom

![Rust](https://github.com/bytebeamio/venom/workflows/Rust/badge.svg)

Tcp tunnels and real world network simulations

Venom is basically our take on a few tools that we really find useful to create robust servers for connected devices

But why?
---------

- Easily simulate 2G, 3G and 4G networks. Networks to simulate dropped packets, halfopen connections, good uplink and bad downlink (and vice versa) connections

- Deal with TRAI [restrictions](https://dot.gov.in/sites/default/files/M2M%20Guidelines.PDF?download=1) on number of IPs in M2M sims by securely tunneling http and tcp data via
a proxy without any changes to the code

- Build robust and light weight tunnels usable in resource constrained embedded device

- Make the life of embedded developers and developers working remotely easy when debugging microservices

- Gain experience in building and maintaining high perf realworld secure servers

Notes
--------

* Network simulators are inspired by toxiproxy. Toxyproxy has some outstanding [bugs](https://github.com/Shopify/toxiproxy/issues/210)
aren't allowing us to move forward

* Ngrok like tunneling but with embedded devices in mind

