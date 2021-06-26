# Build any individual step file.
# Rely on cargo caching instead of doing it via make. Using a fake FORCE
# target to always force cargo to run.
%: src/bin/%.rs .FORCE
	cargo build --release --bin $@

.FORCE: 

.PHONY: .FORCE