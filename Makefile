.PHONY: gen-protoc run build

build:
	cargo build

run:
	cargo run
#	ROCKET_CLI_COLORS=off cargo run

gen-protoc:
	protoc --go_out=plugins=grpc:./src/proto --proto_path=./src/proto --go_opt=paths=source_relative ./src/proto/profanity.proto