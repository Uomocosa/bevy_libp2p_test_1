Objective: I am building a zero-infrastructure, fully decentralized P2P Rust library. I need you to write the complete implementation for the entry point: an asynchronous ping_all_users function.

Technical Constraints & Rules:
1. No Infrastructure: Do not assume the existence of any custom servers, STUN/TURN servers, or custom super-peers.
2. Bootstrap Mechanism: The node must bootstrap itself into the global DHT network using strictly well-known public bootstrap peers (e.g., public IPFS or default libp2p bootstrap nodes).
3. Discovery: Use the Kademlia DHT to find other peers running this exact application. You can use a hardcoded application-specific namespace or topic to identify sibling nodes.
4. Strictly NO mDNS: Disable mDNS entirely. We are targeting real-world WAN behavior, and mDNS pollutes the testing environment.
5. The Ping Protocol: When sibling peers are discovered, the app must connect to them, send a ping message, and successfully receive a pong.
6. Passive/Timeout Logic: If the discovery phase yields no peers, the node must enter a passive listening state, waiting to be discovered and pinged by others (to which it will reply with a pong). If it sits in this passive state without receiving a ping for exactly 60 seconds, the function must timeout and exit gracefully.
