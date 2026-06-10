-include config.mk

SUBDIRS = data

BIN ?= target/debug/noid-welcome
SCRIPTS = \
	oxidize_system \
	system_update \
	virt_manager \
	common

CARGO ?= cargo
CARGO_OPTS ?=

.PHONY: build
build:
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir ; \
	done
	$(CARGO) build $(CARGO_OPTS)

.PHONY: install
install: build
	install -Dm 755 $(BIN) $(DESTDIR)$(BINDIR)/noid-welcome
	install -d $(DESTDIR)$(LIBEXECDIR)/noid-welcome/scripts
	@for script in $(SCRIPTS); do \
		install -m755 scripts/$$script.sh $(DESTDIR)$(LIBEXECDIR)/noid-welcome/scripts; \
	done
	@for dir in $(SUBDIRS); do \
		$(MAKE) install -C $$dir ; \
	done

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(BINDIR)/noid-welcome
	rm -rf $(DESTDIR)$(LIBEXECDIR)/noid-welcome
	rm -rf $(DESTDIR)$(SHAREDIR)/noid-welcome
	@for dir in $(SUBDIRS); do \
		$(MAKE) uninstall -C $$dir ; \
	done

.PHONY: run
run: build
	GTK_THEME=Adwaita-dark $(CARGO) run $(CARGO_OPTS)

.PHONY: clean
clean:
	@for dir in $(SUBDIRS); do \
		$(MAKE) clean -C $$dir ; \
	done
	rm -f src/config.rs
	rm -f config.mk
	@for script in $(SCRIPTS); do \
		rm -f scripts/$$script.sh; \
	done
	$(CARGO) clean
