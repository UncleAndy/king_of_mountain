
build:
		stellar contract build

testnet-deploy: build
	$(eval ADMIN := $(shell stellar keys address deployer))
	$(eval TOKEN := $(shell stellar contract id asset --network testnet --asset native))
	@echo "Admin: $(ADMIN)"
	@echo "Token: $(TOKEN)"
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/king_of_mountain.wasm \
		--source deployer \
		--network testnet \
		-- \
		--admin $(ADMIN) \
		--token_address $(TOKEN)

testnet-withdraw:
	stellar contract invoke \
		--id CCQMOG2ZD7KJH2R52PRVSIBMZLX5XITB4EVUQSMGTZFKHTJUABN2H7TU \
		--source-account deployer \
		--network testnet \
		--\
		 withdraw

mainnet-deploy: build
	$(eval ADMIN := $(shell stellar keys address my-real-admin))
	@echo "Admin: $(ADMIN)"
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/king_of_mountain.wasm \
		--source deployer \
		--rpc-url 'https://soroban-rpc.mainnet.stellar.gateway.fm' \
		--network-passphrase 'Public Global Stellar Network ; September 2015' \
		-- \
		--admin $(ADMIN) \
		--token_address CB2IWR2T3Q7GQPZLVEG7VH5KEMNTNOJNQEZCSN2GF4J4LQSUPRAKJIUP
