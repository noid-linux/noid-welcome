-include ../config.mk

BLP_FILES = \
	window.blp \
	stack/main.blp \
	stack/log.blp \
	stack/get-software.blp
UI_FILES = $(BLP_FILES:.blp=.ui)

resources/ui/%.ui: resources/ui/%.blp
	blueprint-compiler compile --output $@ $<

resources/resources.gresource: resources/resources.gresource.xml $(addprefix resources/ui/, $(UI_FILES))
	glib-compile-resources --sourcedir resources \
		resources/resources.gresource.xml \
		--target resources/resources.gresource

.PHONY: install
install:
	install -Dm 644 com.ch_naseem.NoidWelcome.desktop \
		$(DESTDIR)$(SHAREDIR)/applications/com.ch_naseem.NoidWelcome.desktop
	install -Dm 644 com.ch_naseem.NoidWelcome.metainfo.xml \
		$(DESTDIR)$(SHAREDIR)/metainfo/com.ch_naseem.NoidWelcome.metainfo.xml
	install -Dm 644 resources/resources.gresource \
		$(DESTDIR)$(SHAREDIR)/noid-welcome/resources.gresource
	install -Dm 644 icons/com.ch_naseem.NoidWelcome.png \
		$(DESTDIR)$(SHAREDIR)/icons/hicolor/256x256/apps/com.ch_naseem.NoidWelcome.png

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(SHAREDIR)/applications/com.ch_naseem.NoidWelcome.desktop
	rm -f $(DESTDIR)$(SHAREDIR)/metainfo/com.ch_naseem.NoidWelcome.metainfo.xml
	rm -f $(DESTDIR)$(SHAREDIR)/noid-welcome/resources.gresource
	rm -f $(DESTDIR)$(SHAREDIR)/icons/hicolor/256x256/apps/com.ch_naseem.NoidWelcome.png

.PHONY: clean
clean:
	rm -f $(addprefix resources/ui/, $(UI_FILES))
	rm -rf data/resources/resources.gresource
