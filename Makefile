.PHONY: package

CARCH=x86_64

build: build/authenticator

build/authenticator: src/**.rs Cargo.toml Cargo.lock
	cargo build -Z unstable-options --release --out-dir build --target "${CARCH}-unknown-linux-gnu"

clean:
	rm -rf target
	rm -rf build
	rm -rf package/pkg
	rm -rf package/src
	rm -rf package/*.pkg.*

package:
	cd package && CARCH="${CARCH}" makepkg
