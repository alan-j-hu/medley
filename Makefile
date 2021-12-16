DEBUG ?= 1
ifeq ($(DEBUG), 1)
	INDIR = debug
	OUTDIR = _debug
	CARGO_BUILD = cargo build
else
	INDIR = release
	OUTDIR = _release
	CARGO_BUILD = cargo build --release
endif
PYTHON3 = python3

DUMMY:
	$(CARGO_BUILD) --target wasm32-unknown-unknown

$(OUTDIR)/medley.js: DUMMY
	mkdir -p $(OUTDIR)
	wasm-bindgen --target web --out-dir $(OUTDIR) target/wasm32-unknown-unknown/$(INDIR)/medley_web.wasm

$(OUTDIR)/index.html: web/static/index.html
	mkdir -p $(OUTDIR)
	cp web/static/index.html $(OUTDIR)/index.html

all: $(OUTDIR)/index.html $(OUTDIR)/medley.js

clean:
	rm -r $(OUTDIR)
	cargo clean

run: all
	$(PYTHON3) -m http.server --directory $(OUTDIR)
