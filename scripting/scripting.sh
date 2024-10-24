# Version info.
cargo run --features=cli \
	-- --version

# Help info.
cargo run --features=cli \
	-- --help

# Following the user "frisaf".
cargo run --features=cli \
	-- --mflow \
	--tauth $BLAHAJ_API_TOKEN \
	--apiad "/api" \
	--inadd "https://blahaj.zone" \
	--namei "9upmnr8igmxe01k3"

# Unfollowing the user "frisaf".
cargo run --features=cli \
	-- --dflow \
	--tauth $BLAHAJ_API_TOKEN \
	--apiad "/api" \
	--inadd "https://blahaj.zone" \
	--namei "9upmnr8igmxe01k3"

# Make a new post on my account.
cargo run --features=cli \
	-- --postn \
	--tauth $BLAHAJ_API_TOKEN \
	--apiad "/api" \
	--inadd "https://blahaj.zone" \
	--conte "Posted from the Sharkey.rs CLI." \
	--visie "public" \
	--etype "LikeOnly" 

# Like this note: https://blahaj.zone/notes/9zpo9el4sh0901es
cargo run --features=cli \
	-- --liken \
	--tauth $BLAHAJ_API_TOKEN \
	--apiad "/api" \
	--inadd "https://blahaj.zone" \
	--namei "9zpo9el4sh0901es" \
	--conte "like"

# Unlike this note: https://blahaj.zone/notes/9zpo9el4sh0901es
cargo run --features=cli \
	-- --ulike \
	--tauth $BLAHAJ_API_TOKEN \
	--apiad "/api" \
	--inadd "https://blahaj.zone" \
	--namei "9zpo9el4sh0901es" \
	--conte "like"

