.DEFAULT_GOAL := build
plugins_dir := '/var/lib/coolercontrol/plugins'
executable := 'tuxedo-infinitybook-gen10'
service_id := 'tuxedo-infinitybook-gen10'

.PHONY: clean build install

clean:
	@-$(RM) -rf target
	@-$(RM) -rf vendor

target/release/$(executable):
	@cargo build --locked --release

install: target/release/$(executable)
	@mkdir -p $(DESTDIR)$(plugins_dir)/$(service_id)
	@install -m755 target/release/$(executable) $(DESTDIR)$(plugins_dir)/$(service_id)
	@install -m644 manifest.toml $(DESTDIR)$(plugins_dir)/$(service_id)

run: build
	@sudo target/release/$(executable)

uninstall:
	@-sudo $(RM) -rf $(plugins_dir)/$(service_id)
