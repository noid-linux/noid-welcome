-include config.mk

BIN ?= target/debug/noid-welcome
SCRIPTS = \
	oxidize_system \
	system_update \
	virt_manager \
	common

CARGO ?= cargo
CARGO_OPTS ?=

data/resources/ui/window.ui: data/resources/ui/window.blp
	blueprint-compiler compile --output data/resources/ui/window.ui \
		data/resources/ui/window.blp

data/resources/ui/stack/main.ui: data/resources/ui/stack/main.blp
	blueprint-compiler compile --output data/resources/ui/stack/main.ui \
		data/resources/ui/stack/main.blp

data/resources/ui/stack/log.ui: data/resources/ui/stack/log.blp
	blueprint-compiler compile --output data/resources/ui/stack/log.ui \
		data/resources/ui/stack/log.blp

data/resources/resources.gresource: data/resources/resources.gresource.xml data/resources/ui/window.ui data/resources/ui/stack/main.ui data/resources/ui/stack/log.ui
	glib-compile-resources --sourcedir data/resources \
		data/resources/resources.gresource.xml \
		--target data/resources/resources.gresource

.PHONY: build
build: data/resources/resources.gresource
	$(CARGO) build $(CARGO_OPTS)

.PHONY: install
install: build
	install -Dm 755 $(BIN) $(DESTDIR)$(BINDIR)/noid-welcome
	install -d $(DESTDIR)$(LIBEXECDIR)/noid-welcome/scripts
	@for script in $(SCRIPTS); do \
		install -m755 scripts/$$script.sh $(DESTDIR)$(LIBEXECDIR)/noid-welcome/scripts; \
	done
	install -Dm 644 data/resources/resources.gresource \
		$(DESTDIR)$(SHAREDIR)/noid-welcome/resources.gresource

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(BINDIR)/noid-welcome
	rm -rf $(DESTDIR)$(LIBEXECDIR)/noid-welcome
	rm -rf $(DESTDIR)$(SHAREDIR)/noid-welcome

.PHONY: run
run: build
	GTK_THEME=Adwaita-dark $(CARGO) run $(CARGO_OPTS)

.PHONY: clean
clean:
	rm -f data/resources/ui/window.ui
	rm -rf data/resources/resources.gresource
	rm -f src/config.rs
	rm -f config.mk
	@for script in $(SCRIPTS); do \
		rm -f scripts/$$script.sh; \
	done
	$(CARGO) clean
