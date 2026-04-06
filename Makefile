
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
		--source deployer \
		--network testnet \
		--\
		 withdraw

testnet-update: build
	$(eval NEW_HASH := $(shell stellar contract install \
		--wasm target/wasm32v1-none/release/king_of_mountain.wasm \
		--source deployer \
		--network testnet))
	@echo "Новый хеш кода: $(NEW_HASH)"
	stellar contract invoke \
		--id CAO42C7JEVNEIFWBYDOHC6ERJXKOJMETK2T6HKRJWKJM6UFMXFDCM4OZ \
		--source deployer \
		--network testnet \
		-- \
		upgrade \
		--new_wasm_hash $(NEW_HASH)

mainnet-deploy: build
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/king_of_mountain.wasm \
		--source my-real-admin \
		--rpc-url 'https://soroban-rpc.mainnet.stellar.gateway.fm' \
		--network-passphrase 'Public Global Stellar Network ; September 2015' \
		-- \
		--admin $(shell stellar keys address my-real-admin) \
		--token_address CDUYP3U6HGTOBUNQD2WTLWNMNADWMENROKZZIHGEVGKIU3ZUDF42CDOK

mainnet-update:
	$(eval NEW_HASH := $(shell stellar contract install \
		--wasm target/wasm32v1-none/release/king_of_mountain.wasm \
		--source my-real-admin \
		--rpc-url 'https://soroban-rpc.mainnet.stellar.gateway.fm' \
		--network-passphrase 'Public Global Stellar Network ; September 2015'))
	@echo "Новый хеш кода: $(NEW_HASH)"
	#stellar contract invoke \
	#	--id CAFXUALXFPTBTLSRCDSMJXNPSN3AVL2ZPXJUDDHVTUTLRX5SCNP2SISM \
	#	--source my-real-admin \
	#	--rpc-url 'https://soroban-rpc.mainnet.stellar.gateway.fm' \
	#	--network-passphrase 'Public Global Stellar Network ; September 2015' \
	#	-- \
	#	upgrade \
	#	--new_wasm_hash $(NEW_HASH)

