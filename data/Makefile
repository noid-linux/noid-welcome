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

.PHONY: clean
clean:
	rm -f $(addprefix resources/ui/, $(UI_FILES))
	rm -rf data/resources/resources.gresource
