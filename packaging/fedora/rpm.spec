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
cargo build

%install
# install -Dpm 0755 target/rpm/tuxedo-infinitybook-gen10 -t %{buildroot}%{_bindir}

%files
%license LICENSE
%doc README.md
# %{_bindir}/tuxedo-infinitybook-gen10

%changelog
%autochangelog
