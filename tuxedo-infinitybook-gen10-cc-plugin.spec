Name:           tuxedo-infinitybook-gen10-cc-plugin
Version:        0.1.0
Release:        %autorelease
Summary:        Plugin for CoolerControl for Tuxedo InfinityBook Gen10 laptops

SourceLicense:  GPLv3
License:        GPLv3

URL:            https://github.com/sagebind/tuxedo-infinitybook-gen10-cc-plugin
Source:         %{url}/archive/refs/tags/%{version}.tar.gz

BuildRequires:  cargo-rpm-macros

%description
Plugin for CoolerControl for Tuxedo InfinityBook Gen10 laptops.

%prep
%autosetup -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires -t

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
install -Dpm 0755 target/rpm/tuxedo-infinitybook-gen10 -t %{buildroot}%{_bindir}

%check
%cargo_test

%files
%license LICENSE
%doc README.md
%{_bindir}/tuxedo-infinitybook-gen10

%changelog
%autochangelog
