VERSION := $(shell awk -F'"' '/^version = / {print $$2; exit;}' < Cargo.toml)
SOURCE_NAME := tuxedo-infinitybook-gen10-cc-plugin-$(VERSION)
SOURCE_ARCHIVE := $(SOURCE_NAME).tar.gz
TAR_ARGS := --exclude=.git --exclude=target

srpm:
	mkdir -p $(outdir)
	tar czf $(outdir)/$(SOURCE_ARCHIVE) $(TAR_ARGS) --transform='s,^.,$(SOURCE_NAME),' .
	rpmbuild -bs \
		--define "_sourcedir $(outdir)" \
		--define "_srcrpmdir $(outdir)" \
		packaging/fedora/rpm.spec
