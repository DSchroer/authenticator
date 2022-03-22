build: build/authenticator

CARGO_ARGS=
build/authenticator: src/**.rs Cargo.toml Cargo.lock
	cargo build -Z unstable-options --release --out-dir build ${CARGO_ARGS}

install:
	cp build/authenticator /usr/local/bin/
	cp ui/authenticator_ui.sh /usr/local/bin/
	cp ui/authenticator.desktop /usr/share/applications

uninstall:
	rm /usr/local/bin/authenticator
	rm /usr/local/bin/authenticator_ui.sh
	rm /usr/share/applications/authenticator.desktop

clean:
	rm -rf target
	rm -rf build
