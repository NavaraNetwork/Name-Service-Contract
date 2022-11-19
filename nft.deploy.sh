cd nft-contract

sh build.sh

cd ..

near deploy \
    --wasmFile out/nft-domain.wasm \
    --initFunction "new" \
    --initArgs '{
        "metadata": {
            "spec": "nft-1.0.0",
            "name": "Dnet NFT",
            "symbol": "DNFT",
            "icon": "https://bafybeie4sb64l2tevfwrt6dtr7z32seliyybw4g5nanxt4dbdokn2fyk6m.ipfs.dweb.link/dnet.png"
        },
        "owner_id": "nft.gnet.testnet"
    }' \
    --accountId nft.gnet.testnet