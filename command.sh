# mint domain
near call nft.gnet.testnet nft_mint '{ "token_id": "manhng.btc", "metadata": { "title": "manhng.btc", "description": "bitcoin domain provided by decentrailnet", "media": "https://vcdn-sohoa.vnecdn.net/2022/03/08/bored-ape-nft-accidental-0-728-5490-8163-1646708401.jpg" }, "receiver_id": "manhng.testnet" }' --accountId manhng.testnet --amount 0.2

# get address buy token id and network
near view nft.gnet.testnet get_address '{"token_id": "manhng.btc", "network": "Ethereum"}'

# get addrsses added to token id
near view nft.gnet.testnet get_token_addresses '{"token_id": "manhng.btc"}'

# insert address to a domain (token)
near call nft.gnet.testnet insert_addresses '{"token_id": "manhng.btc", "addresses_input": [{"network": "Ethereum", "address": "0x33ed1b1B29e807fCf15EC731b0c0DE18d306be1a"}]}' --deposit 0.003 --accountId manhng.testnet

# remove address 
near near call nft.gnet.testnet insert_addresses '{"token_id": "manhng.btc", "addresses_input": [{"network": "Ethereum"}]}' --accountId manhng.testnet

# reset address
near call nft.gnet.testnet reset_token_addresses '{"token_id": "manhng.btc"}' --accountId manhng.testnet

# extend token
near call nft.gnet.testnet extend_token '{"token_id": "manhng.btc"}' --accountId manhng.testnet --deposit 0.6