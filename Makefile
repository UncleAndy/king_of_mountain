
build:
		stellar contract build

testnet-deploy: build
	@contract_id=$$(stellar contract deploy \
		--wasm target/wasm32v1-none/release/king_of_mountain.wasm \
		--source deployer \
		--network testnet) && \
	echo "Deployed Contract ID: $$contract_id" && \
	stellar contract invoke \
		--id $$contract_id \
		--source deployer \
		--network testnet \
		-- \
		init \
		--admin $$(stellar keys address deployer) \
		--token_address $$(stellar contract id asset --network testnet --asset native)

mainnet-deploy: build
	@contract_id=$$(stellar contract deploy \
		--wasm target/wasm32v1-none/release/king_of_mountain.wasm \
		--source my-real-admin \
		--rpc-url 'https://soroban-rpc.mainnet.stellar.gateway.fm' \
		--network-passphrase 'Public Global Stellar Network ; September 2015') && \
	echo "Deployed Contract ID: $$contract_id" && \
	stellar contract invoke \
		--id $$contract_id \
		--source my-real-admin \
		--rpc-url 'https://soroban-rpc.mainnet.stellar.gateway.fm' \
		--network-passphrase 'Public Global Stellar Network ; September 2015' \
		-- init \
		--admin $$(stellar keys address my-real-admin) \
		--token_address CB2IWR2T3Q7GQPZLVEG7VH5KEMNTNOJNQEZCSN2GF4J4LQSUPRAKJIUP
