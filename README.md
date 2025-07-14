# Jito Shredstream Proxy

ShredStream provides the lowest latency to shreds from leaders on Solana. 

See more at https://docs.jito.wtf/lowlatencytxnfeed/

## Disclaimer
Use this at your own risk.

RUST_LOG=info ./jito-shredstream-proxy shredstream \
    --block-engine-url https://mainnet.block-engine.jito.wtf \
    --auth-keypair my_keypair.json \
    --desired-regions amsterdam,ny \
    --grpc-service-port 9999 \
    --dest-ip-ports 127.0.0.1:8002,65.109.87.35:8002

ldd (Ubuntu GLIBC 2.39-0ubuntu8.4) 2.39