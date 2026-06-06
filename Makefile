data/resources/ui/window.ui: data/resources/ui/window.blp
	blueprint-compiler compile --output data/resources/ui/window.ui \
		data/resources/ui/window.blp

data/resources/resources.gresource: data/resources/resources.gresource.xml data/resources/ui/window.ui
	glib-compile-resources --sourcedir data/resources \
		data/resources/resources.gresource.xml \
		--target data/resources/resources.gresource

.PHONY: build
build: data/resources/resources.gresource
	cargo build

.PHONY: run
run: build
	GTK_THEME=Adwaita-dark SCRIPTS_DIR=$(CURDIR)/scripts cargo run -q

.PHONY: clean
clean:
	rm -f data/resources/ui/window.ui
	rm -rf data/resources/resources.gresource
	cargo clean
