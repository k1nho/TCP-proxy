build: 
	mkdir prod;
	cp config.json prod/
	cargo build --release
	cp ./target/release/tcp_proxy ./prod/

run: 
	prod/tcp_proxy

test:
	cd test_proxy; go test -v

clean: 
	rm -rf prod


all: build run
