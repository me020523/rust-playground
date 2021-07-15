Rust_APP_Name := app
Rust_APP_Files := $(wildcard src/*.rs)

ifeq ($(MITIGATION-CVE-2020-0551), LOAD)
export MITIGATION_CVE_2020_0551=LOAD
else ifeq ($(MITIGATION-CVE-2020-0551), CF)
export MITIGATION_CVE_2020_0551=CF
endif

.PHONY: all

all: $(Rust_APP_Name)

$(Rust_APP_Name): $(Rust_APP_Files)
	cargo build --release
	cp ./target/release/app ../bin/app
