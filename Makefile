APPID = dev.trytonvanmeer.Steam.SearchProvider

DESTDIR =
PREFIX = /usr/local

BINDIR = $(PREFIX)/bin
USERUNITDIR = $(PREFIX)/lib/systemd/user
DATADIR = $(PREFIX)/share
DBUS_SERVICES_DIR = $(DATADIR)/dbus-1/services
SEARCH_PROVIDERS_DIR = $(DATADIR)/gnome-shell/search-providers

.PHONY: build install log service-start service-restart

build:
	cargo build --release --locked

install:
	install -Dm644 -t $(DESTDIR)$(SEARCH_PROVIDERS_DIR) providers/${APPID}.ini
	install -Dm644 -t $(DESTDIR)$(USERUNITDIR) systemd/${APPID}.service
	install -Dm644 -t $(DESTDIR)$(DBUS_SERVICES_DIR) dbus-1/${APPID}.service
	install -Dm755 -t $(DESTDIR)$(BINDIR) target/release/gnome-search-provider-steam

log:
	journalctl --user --follow --unit ${APPID}

service-restart:
	systemctl --user daemon-reload && systemctl --user restart ${APPID}
