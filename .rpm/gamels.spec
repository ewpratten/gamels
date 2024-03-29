%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: gamels
Summary: List current directory, converting file names to steam game titles
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPL-3.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/ewpratten/gamels

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
