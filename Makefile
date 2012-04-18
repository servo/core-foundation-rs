RUSTC ?= rustc
RUSTFLAGS ?=

RUST_SRC = $(shell find . -type f -name '*.rs')

.PHONY: all
all: libcocoa.dummy

libcocoa.dummy: cocoa.rc $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $< -o $@
	touch $@

cocoa-test: cocoa.rc $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $< -o $@ --test

check: cocoa-test
	./cocoa-test

.PHONY: clean
clean:
	rm -f cocoa-test *.so *.dylib *.dll *.dummy
