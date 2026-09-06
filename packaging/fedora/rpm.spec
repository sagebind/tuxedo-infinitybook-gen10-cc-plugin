Name:           tuxedo-infinitybook-gen10-cc-plugin
Version:        0.0.0
Release:        0%{?dist}
Summary:        Plugin for CoolerControl for Tuxedo InfinityBook Gen10 laptops

License:        GPLv3

URL:            https://github.com/sagebind/tuxedo-infinitybook-gen10-cc-plugin
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  protobuf

%description
Plugin for CoolerControl for Tuxedo InfinityBook Gen10 laptops.

%prep
%setup

%build
make build

%install
make DESTDIR=%{buildroot} install

%files
%license LICENSE
%doc README.md
/var/lib/coolercontrol/plugins/tuxedo-infinitybook-gen10/manifest.toml
/var/lib/coolercontrol/plugins/tuxedo-infinitybook-gen10/tuxedo-infinitybook-gen10

%changelog
%autochangelog
