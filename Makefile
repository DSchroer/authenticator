build: build/authenticator

CARGO_ARGS=
build/authenticator: src/**.rs Cargo.toml Cargo.lock
	cargo build -Z unstable-options --release --out-dir build ${CARGO_ARGS}

install:
	cp build/authenticator /usr/local/bin/
	cp ui/authenticator_ui.sh /usr/local/bin/
	cp ui/ca.schroer.authenticator.desktop /usr/share/applications
	cp ui/ca.schroer.authenticator.png /usr/share/icons/hicolor/128x128/apps

uninstall:
	rm /usr/local/bin/authenticator
	rm /usr/local/bin/authenticator_ui.sh
	rm /usr/share/applications/ca.schroer.authenticator.desktop
	rm /usr/share/icons/hicolor/128x128/apps/ca.schroer.authenticator.png

clean:
	rm -rf target
	rm -rf build
