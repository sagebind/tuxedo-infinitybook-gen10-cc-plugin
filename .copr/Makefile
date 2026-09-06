srpm:
	rpmdev-setuptree
	rm ~/rpmbuild/SOURCES/tuxedo-infinitybook-gen10-cc-plugin-0.0.0.tar.gz
	git ls-files | tar Tczf - ~/rpmbuild/SOURCES/tuxedo-infinitybook-gen10-cc-plugin-0.0.0.tar.gz --transform 's,^,tuxedo-infinitybook-gen10-cc-plugin-0.0.0/,'
	rpmbuild -bs packaging/fedora/rpm.spec
	mkdir -p $(outdir)
	mv ~/rpmbuild/SRPMS/tuxedo-infinitybook-gen10-cc-plugin-*.src.rpm $(outdir)
