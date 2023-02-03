build: 
	mkdir prod;
	cp config.json prod/
	cargo build --release
	cp ./target/release/tcp_proxy ./prod/

run: 
	prod/tcp_proxy

clean: 
	rm -rf prod


all: build run
