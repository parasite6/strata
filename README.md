<div align="center">

<img src="docs/assets/logos/strata-tokyo-night.svg" alt="Strata logo" width="160">

# Strata (unofficial fork)

**Navigate every layer.** A fast, keyboard-first file manager for modern Linux desktops.

[![Fork CI](https://github.com/parasite6/strata/actions/workflows/fork-ci.yml/badge.svg?branch=main)](https://github.com/parasite6/strata/actions/workflows/fork-ci.yml)
[![Upstream release](https://img.shields.io/github/v/release/lgse/strata?display_name=tag&sort=semver&label=upstream%20release)](https://github.com/lgse/strata/releases/latest)
[![License: MIT](https://img.shields.io/github/license/lgse/strata)](LICENSE)
[![Linux](https://img.shields.io/badge/platform-Linux-FCC624?logo=linux&logoColor=black)](#technical-specifications)

<picture>
  <source media="(prefers-reduced-motion: no-preference)" srcset="docs/assets/strata-demo.gif">
  <img src="docs/assets/strata-columns.png" alt="Strata showing Columns, Icons, and List views, search, a context menu, file properties, and settings" width="1280">
</picture>

<sub>The animation respects reduced-motion preferences. View the [static preview](docs/assets/strata-columns.png).</sub>

</div>

> [!IMPORTANT]
> **This is not the official Strata repository.** It is an unofficial fork of
> [lgse/strata](https://github.com/lgse/strata), maintained independently by
> [parasite6](https://github.com/parasite6). It is not affiliated with or endorsed by
> LGSE Ltd. or the Strata maintainers. For the official project, releases and support,
> go to **[github.com/lgse/strata](https://github.com/lgse/strata)**. Please don't
> report problems with this fork to the official project.

Strata combines spatial Miller-column navigation with familiar Icons and List views, instant fuzzy filename search, rich previews, and native Linux desktop integration. It is designed for Omarchy and works on compatible GTK4 Linux environments.

## Contents

- [About this fork](#about-this-fork)
- [Features](#features)
- [Installation](#installation)
  - [Install with the installer](#install-with-the-installer)
  - [Build manually](#build-manually)
  - [Nix and NixOS](#nix-and-nixos)
  - [Update](#update)
  - [Uninstall](#uninstall)
- [Usage and desktop integration](#usage-and-desktop-integration)
  - [Desktop entry](#desktop-entry)
  - [Make Strata the Omarchy file manager](#make-strata-the-omarchy-file-manager)
  - [Unlock encrypted volumes on Omarchy](#unlock-encrypted-volumes-on-omarchy)
  - [Network shares](#network-shares)
- [Custom actions and script authoring](#custom-actions-and-script-authoring)
- [Theming](#theming)
  - [Follow Omarchy Quattro](#follow-omarchy-quattro)
  - [Bundled themes](#bundled-themes)
  - [Custom themes](#custom-themes)
- [Release channels](#release-channels)
- [Under the hood](#under-the-hood)
- [Technical specifications](#technical-specifications)
- [Development and documentation](#development-and-documentation)
- [Credits](#credits)
- [License](#license)

## About this fork

This repository is a branch of the official Strata project at
[github.com/lgse/strata](https://github.com/lgse/strata). Almost all of the code,
design and documentation here is the work of the upstream authors; see
[Credits](#credits).

- **Kept in sync with upstream.** A scheduled job merges upstream's `main` into this
  fork every day, so upstream fixes and features arrive here too.
- **No official releases.** This fork publishes no binaries or signed releases. Install
  it by building from source, as described in [Installation](#installation).
  Release downloads, the signed update manifest and the in-app updater all refer to
  the official project.
- **Changes are not sent upstream.** Fork changes are made directly on this
  repository's `main` and are not proposed to the official project.

What this fork adds on top of upstream:

- **Compression options:** a compression level from Store to Ultra for ZIP, 7Z and
  TAR.GZ, and an **Encrypt file names** choice for password-protected 7Z archives.
- **SHA-256 in Properties:** a file's checksum, calculated in the background and
  selectable for copying.
- **Folder previews in Icons view:** folders show thumbnails of the pictures and
  videos inside them, with a **Folder previews** setting; network shares are skipped.
- **Icon themes:** file icons from installed icon themes or your own images, following
  desktop theme changes live.
- **Media players:** a per-stream loop toggle.
- **File chooser:** a single click selects instead of previewing, and Save dialogs open
  with nothing selected.
- **Text preview:** a wider, classic editor scrollbar.
- **Source installs:** `install.sh` builds this fork from a checkout, with Fedora
  support. See [Installation](#installation).
- **Fixes:** stale glyph pixels left after text changes, path bar and pane header
  repaints, filter results that lost their preview, the partial-search notice covering
  results, and faster listing of large folders.

## Features

- **Three browser modes:** navigable Columns, an Icons grid, and a sortable List table.
- **Keyboard-first control:** directional-key movement, navigation history, location entry, pane filtering, fuzzy search, file operations, and quick previews. An optional footer and F1 shortcut reference help you learn each mode; the footer also shows clipboard status. See [keyboard navigation and paste destinations](docs/keyboard-navigation.md). Optional [10xer mode](docs/10xer-mode.md) hides window and pane chrome and uses Yazi-style keys. Change, add, or remove shortcuts, separately for each mode, in **Settings → Keyboard**; see [Keymap](docs/keymap.md).
- **Fast recursive search:** press <kbd>Ctrl</kbd>+<kbd>K</kbd> to find files and directories by name or path while the tree is still being indexed. Global search covers Home and the mounted local drives listed under **Devices** in the sidebar, regardless of the current folder; system mounts that Devices hides, such as a btrfs subvolume for a package cache, are not searched. The dialog warns when results are incomplete; raise or remove the indexing time and entry limits under **Settings → General → Search & filtering** for slow or network mounts. Folder-scoped filtering/search remains separate. URI-native remote shares are not yet included.
- **Rich previews and thumbnails:** native rendered Markdown and static HTML, plus bounded previews for text, source code, images, camera RAW, PDF, audio, and video, with native parser-backed formats isolated from the application. Audio opens in a now-playing view with artwork and a live spectrum; video gets technical badges, a keyframe storyboard while scrubbing, ambient light around the frame, silent autoplay that eases its sound in, and <kbd>Enter</kbd> hands the position to your default player. File Properties shows available media resolution, duration, bitrate, codecs, HDR, subtitle and chapter counts, and audio/video rates. RAW previews and Properties also show dimensions, camera, lens, focal length, shutter speed, ISO, and GPS coordinates, with `N/A` for unavailable values.
- **Drives kept together in Devices:** each drive's volumes stay next to each other in the system's drive order, with fixed disks before removable ones, a divider between drives, and network shares last.
- **Shared storage properties:** volumes and mounted storage entries in Devices use the same Properties dialog for name, device, filesystem, mount status, and available capacity/usage. **Set label…** saves a display label only in Strata; it never changes the filesystem label, mounts/unmounts a drive, or alters system configuration. Labels update the sidebar and Properties live and can be cleared to restore the normal device name. Formatting remains restricted to eligible removable drives; release actions follow the device's reported capabilities.
- **Responsive filesystem work:** cancellable directory loading, bounded streaming, incremental monitoring, stable selection, and virtualized large directories.
- **Everyday file operations:** create folders, rename, cut, copy, paste, trash, permanent delete, sorting, hidden files, pins, and history. [Drop images, selected text, and web links](docs/content-drops.md) from a browser into a folder to save them as files; a dropped link is saved as a link file and never fetched.
- **Remote locations:** browse GIO/GVfs locations such as authenticated SMB shares and SFTP servers from the location field, with explicit host-key decisions.
- **Administrator access:** open a local folder you can't read, such as a root-owned backup folder, after the system's polkit password prompt. See [Administrator access](docs/administrator-access.md).
- **Adaptive appearance:** compact or airy density, 95 bundled themes, custom themes, and live Omarchy Quattro theme following.
- **Updates in the app:** automatic update checks (on by default; turn them off in **Settings → Updates**), release notes, verified downloads, and in-place installation for release binaries. In this fork these offer the **official** releases, which replace the fork's build; see [Installation](#installation).
- **Custom actions:** add your own scripts to the file and folder context menus, with a manager in **Settings → Actions** and background progress in the Jobs dashboard. See [Custom actions](docs/custom-actions.md).
- **Integrations:** opt-in providers maintained in the repository's [`integrations/`](integrations/README.md) catalog, installed, updated and removed from **Settings → Integrations**; they plug services and devices into the context menu and the **Send to…** menu. See [External file providers](docs/file-providers.md).
- **System file chooser:** opt in through **Settings → General → Desktop integration → System file manager**, the installer, or `strata --install-portal`; see [portal setup](docs/portal-file-chooser.md).
- **Encrypted-volume unlock:** opt in through the installer, **Settings → General → Desktop integration** on Omarchy, or `strata --install-udiskie-unlock`; restore with `strata --uninstall-udiskie-unlock` (Settings **Restore default** on Omarchy only).

## Installation

This fork has **no prebuilt releases, COPR package, AUR package or Nix cache**. It is
always built from source on your machine. The packages, release archives and installer
commands published by the official project install the **official** Strata, not this
fork; for those, see the [official installation guide](https://github.com/lgse/strata#installation).

Arch Linux, Omarchy and Fedora are supported by the installer. Strata needs a
glibc-based Linux system with GTK **4.12 or newer**. On other distributions, install
the equivalent packages yourself and [build manually](#build-manually).

> [!WARNING]
> **Settings → Updates installs the official release** from lgse/strata and replaces
> this fork's build, and so does the installer when run outside a checkout without
> `--from-source`, or with `--from-release`. To stay on the fork, turn off automatic
> update checks in **Settings → Updates** and update as described in
> [Update](#update).

<a id="interactive-installation"></a>

### Install with the installer

The installer installs the build and runtime packages, Rust included, with pacman
(Arch, Omarchy) or dnf (Fedora), builds this fork with Cargo, and installs it
per-user at `~/.local/bin/strata` with the desktop integrations you choose. The build
runs at low priority on half your CPU cores; set `STRATA_BUILD_JOBS` to change that.
A first build takes several minutes.

From a checkout, which is the easiest way to update later:

```bash
git clone https://github.com/parasite6/strata.git
cd strata
./install.sh
```

Or in one step, which clones this fork into a temporary folder and builds it there:

```bash
curl -fsSL https://raw.githubusercontent.com/parasite6/strata/main/install.sh | bash -s -- --from-source
```

The recommended setup installs every integration your system supports: the
application launcher, folder handling and "Open file location", the system file
chooser, SMB and image/RAW support, the file-manager shortcuts on Omarchy 3 or 4 (a
backup of the previous bindings is kept), and encrypted-volume unlock when `udiskie`
is installed. Choose **Customize installation** to deselect any of them; only options
your system supports are offered. Use the arrow keys and Enter to choose, and Space to
toggle custom options. The installer shows every privileged package operation before
running it, and `--verbose` prints detailed output.

For an unattended installation, pass `--non-interactive`, and decline individual
integrations with `--without-*` flags:

```bash
./install.sh --non-interactive --without-smb --without-file-chooser
```

Each `--with-*` or `--without-*` flag implies `--non-interactive`. A `--with-*` flag
fails before installing anything when the system cannot provide that integration,
whereas a default is skipped with a warning. `--with-omarchy-keybinds` is accepted
only on Omarchy 3 or 4, and `--with-udiskie-unlock` only when `udiskie` is already on
`PATH` on an Arch-based system. Folder association implies the desktop entry and "Open
file location". `--without-file-chooser` keeps your current chooser and dismisses the
one-time in-app offer. The installer never installs the `udiskie` package.
Non-interactive package installation needs passwordless sudo or cached credentials.
Run `./install.sh --help` for every option.

Phone backends are not installed. For iPhone, iPad or Android access, follow
[Connecting phones](#connecting-phones) after installation.

<a id="manual-installation"></a>

### Build manually

Install the build tools and libraries, and the runtime packages.

On Arch Linux or Omarchy:

```bash
sudo pacman -S --needed base-devel git pkgconf rust \
  bubblewrap desktop-file-utils ffmpeg ffmpegthumbnailer fontconfig gstreamer gst-libav \
  gst-plugins-base gst-plugins-good gtk4 gtksourceview5 gvfs poppler-glib xdg-utils
# Optional SMB, AppImage icon, and broader camera RAW support:
sudo pacman -S --needed gvfs-smb imagemagick libraw dcraw squashfs-tools
```

On Fedora:

```bash
sudo dnf install cargo rust gcc git pkgconf-pkg-config fontconfig-devel glib2-devel \
  gstreamer1-devel gstreamer1-plugins-base-devel gtk4-devel gtksourceview5-devel \
  poppler-glib-devel \
  bubblewrap desktop-file-utils ffmpeg-free ffmpegthumbnailer fontconfig gstreamer1 \
  gstreamer1-plugin-libav gstreamer1-plugins-base gstreamer1-plugins-good gtk4 \
  gtksourceview5 gvfs hicolor-icon-theme poppler-glib xdg-utils
# Optional SMB, AppImage icon, and broader camera RAW support:
sudo dnf install gvfs-smb ImageMagick LibRaw-samples dcraw squashfs-tools
```

`ffmpeg` from RPM Fusion can replace `ffmpeg-free` for more preview formats.

Then build and install the binary:

```bash
git clone https://github.com/parasite6/strata.git
cd strata
cargo build --release --locked
install -Dm755 target/release/strata ~/.local/bin/strata
strata
```

If `strata` is not found, add `~/.local/bin` to your shell's `PATH`. A manual build has
no desktop entry; run `./install.sh` from the same checkout to add the integrations,
or see [Desktop entry](#desktop-entry). See
[Development and documentation](#development-and-documentation) for build options,
such as building without RAR support or without the in-app updater.

Device discovery needs the GVfs UDisks2 volume monitor (`gvfs` on Arch and Fedora,
`gvfs-daemons` on Debian and Ubuntu); without it, removable drives may be missing from
Devices. If previews, thumbnails, network locations or drives don't work, open
**Settings → Diagnostics** and select **Run**. It names any missing package with an
install command for your distribution, and **Copy report** copies the results.

### Nix and NixOS

The repository's flake builds from source, so it can build this fork on `x86_64-linux`
or `aarch64-linux`:

```bash
nix run github:parasite6/strata
nix profile add github:parasite6/strata
```

For NixOS or Home Manager, add `inputs.strata.url = "github:parasite6/strata";` and
follow the official
[Nix and NixOS instructions](https://github.com/lgse/strata#nix-and-nixos) for the
package, the file chooser portal, the FileManager1 service and udiskie, which apply
unchanged. Update the flake input and rebuild to upgrade.

### Update

From your checkout:

```bash
git pull
./install.sh
```

For a manual build, run `git pull`, `cargo build --release --locked`, and the
`install` command again. A one-step installation has no checkout to update; run the
one-step command again.

### Uninstall

If you opted into encrypted-volume unlock, run `strata --uninstall-udiskie-unlock`
first. Then remove the per-user installation:

```bash
rm -f ~/.local/bin/strata \
  ~/.local/share/applications/io.github.lgse.Strata.desktop \
  ~/.local/share/dbus-1/services/io.github.lgse.Strata.FileManager1.service \
  ~/.local/share/icons/hicolor/scalable/apps/io.github.lgse.Strata.svg
update-desktop-database ~/.local/share/applications 2>/dev/null || true
gtk-update-icon-cache -qtf ~/.local/share/icons/hicolor 2>/dev/null || true
```

Preferences and custom themes stay under your XDG configuration directories, so
uninstalling keeps your settings.

## Usage and desktop integration

Launch Strata with an optional local directory:

```bash
strata                      # home directory
strata ~/Documents          # a specific directory
strata --unlock-volume DEVICE  # unlock an encrypted volume
strata --install-udiskie-unlock   # install Strata as the udiskie unlock handler
strata --uninstall-udiskie-unlock # restore the previous udiskie configuration
strata --version            # print the installed version
```

In the default map, useful shortcuts include <kbd>Ctrl</kbd>+<kbd>K</kbd> for recursive search, <kbd>Ctrl</kbd>+<kbd>L</kbd> for a path or URI, <kbd>Ctrl</kbd>+<kbd>F</kbd> to filter the current pane, <kbd>Ctrl</kbd>+<kbd>Z</kbd> to undo or <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd> to redo the latest reversible file operation, <kbd>Space</kbd> for preview, <kbd>F2</kbd> to rename, and <kbd>Alt</kbd>+arrow keys for history and parent navigation.

Toggle [10xer mode](docs/10xer-mode.md) with <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>M</kbd> for Yazi-style navigation and footer prompts; press it again to return to the default map. In 10xer mode, <kbd>l</kbd>/<kbd>→</kbd> enters preview, <kbd>h</kbd> returns to the listing without closing it, and <kbd>i</kbd> toggles it.

In narrow or highly scaled windows, the file view takes priority: the preview shrinks, then hides when there is not enough room for both. Widening the window restores the preview and its session width automatically. Hidden previews pause playback and defer loading new selections; unchanged previews return without reloading the file.

### Desktop entry

The installer adds a per-user desktop entry and application icon unless you deselect
them. To add them yourself after a [manual build](#build-manually), run from your
checkout:

```bash
install -Dm644 data/icons/scalable/apps/io.github.lgse.Strata.svg \
  ~/.local/share/icons/hicolor/scalable/apps/io.github.lgse.Strata.svg
install -d ~/.local/share/applications
sed "s|^Exec=strata |Exec=$HOME/.local/bin/strata |" data/io.github.lgse.Strata.desktop \
  > ~/.local/share/applications/io.github.lgse.Strata.desktop
update-desktop-database ~/.local/share/applications
gtk-update-icon-cache -qtf ~/.local/share/icons/hicolor 2>/dev/null || true
xdg-mime default io.github.lgse.Strata.desktop inode/directory
xdg-mime query default inode/directory
```

The final command should print `io.github.lgse.Strata.desktop`, which also makes Strata
the default folder handler. The entry's filename matches the `io.github.lgse.Strata`
application ID that Strata's windows report, so desktop shells match a running window
to it and show its icon. This fork keeps the official application ID. Log out and back
in if a shell caches launcher icons.

With [mise](https://mise.jdx.dev), `mise run install-local` builds and installs the
binary, icon and desktop entry in the same locations, and `mise run uninstall-local`
removes them.

### "Open file location" from other applications

Browsers and GTK/GNOME applications reveal a file by calling the
`org.freedesktop.FileManager1` D-Bus interface instead of consulting the
`inode/directory` association. The installer offers this separately; for unattended
installation, pass `--with-file-manager`. Folder association enables it automatically.

To enable it yourself, run `mise run install-file-manager` from your checkout, or
install the service file directly:

```bash
install -d ~/.local/share/dbus-1/services
sed "s|^Exec=/usr/bin/strata |Exec=$HOME/.local/bin/strata |" \
  data/io.github.lgse.Strata.FileManager1.service \
  > ~/.local/share/dbus-1/services/io.github.lgse.Strata.FileManager1.service
```

A per-user provider takes precedence over system providers shipped by other file managers. Before enabling Strata manually, remove any other per-user service whose `Name` is `org.freedesktop.FileManager1`; two providers for the same name in one service directory are chosen arbitrarily. If another file manager already owns the bus name, exit it before testing. Use `mise run uninstall-file-manager`, or remove the per-user service file, to disable Strata again.

Strata then answers `ShowFolders`, `ShowItems`, and `ShowItemProperties`, opening the directory that holds the named items with those items selected:

```bash
busctl --user call org.freedesktop.FileManager1 /org/freedesktop/FileManager1 \
  org.freedesktop.FileManager1 ShowItems ass 1 "file://$HOME/Downloads" ""
```

### Make Strata the Omarchy file manager

The XDG association above handles folders opened by applications. On current Lua-based Omarchy releases, also override the stock Nautilus shortcuts in `~/.config/hypr/bindings.lua` so Omarchy launches Strata directly.

First inspect the active bindings and back up your user configuration:

```bash
omarchy menu keybindings --print | grep -i "file manager"
cp ~/.config/hypr/bindings.lua ~/.config/hypr/bindings.lua.bak.$(date +%s)
```

Append these overrides to `~/.config/hypr/bindings.lua`:

```lua
-- Use Strata instead of Nautilus for Omarchy's file-manager shortcuts.
hl.unbind("SUPER + SHIFT + F")
hl.unbind("SUPER + ALT + SHIFT + F")
o.bind("SUPER + SHIFT + F", "File manager", { launch = "strata" })
o.bind("SUPER + ALT + SHIFT + F", "File manager (cwd)",
  "uwsm-app -- strata \"$(omarchy-cmd-terminal-cwd)\"")
```

The stock shortcut is <kbd>Super</kbd>+<kbd>Shift</kbd>+<kbd>F</kbd>, not <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>F</kbd>. To support the Ctrl chord too, first confirm that it is not assigned to another action, then optionally append:

```lua
o.bind("CTRL + SHIFT + F", "File manager", { launch = "strata" })
```

Apply and validate the configuration:

```bash
hyprctl reload
hyprctl configerrors
omarchy menu keybindings --print | grep -i "file manager"
```

`hyprctl configerrors` should produce no errors. These user overrides survive Omarchy updates; do not edit files under `/usr/share/omarchy/`.

### Unlock encrypted volumes on Omarchy

On Omarchy 3 or 4, the installer asks whether to use Strata for encrypted-volume unlock (default No). **Settings → General → Desktop integration** includes **Unlock encrypted volumes**. Choose **Use Strata** so plugging in an encrypted drive opens Strata's password prompt instead of udiskie's dialog. **Restore default** returns udiskie to its previous configuration. When this version of Strata or a later one made the setup and udiskie's settings have not changed since, it puts back the original file byte for byte, comments and formatting included; otherwise it keeps your later changes and removes only Strata's.

On generic Arch, the installer offers the same step only when `udiskie` is already on `PATH`. Restore there with `strata --uninstall-udiskie-unlock`; Settings for this row stays Omarchy-only.

The same setup is available unattended:

```bash
strata --install-udiskie-unlock
strata --uninstall-udiskie-unlock
```

This edits `~/.config/udiskie/config.yml` and records restore state in `~/.local/share/strata/udiskie-install/state.toml`. It is not a saved Strata preference. The Settings row appears only when Omarchy is detected and `udiskie` is on your `PATH`; the CLI flags are unattended and are not Omarchy-gated. Unattended installer opt-in is `--with-udiskie-unlock`.

### Connecting phones

Strata discovers and mounts phones through GIO/GVfs. The required phone backends
are optional and are not installed by `install.sh`. On Arch Linux or Omarchy,
install only the support you need:

```bash
# iPhone or iPad app documents (AFC):
sudo pacman -S --needed gvfs-afc usbmuxd
# Camera/PTP photo access, including compatible iPhones:
sudo pacman -S --needed gvfs-gphoto2
# Android (MTP):
sudo pacman -S --needed gvfs-mtp
```

Other distributions need equivalent GVfs AFC, gphoto2/PTP, or MTP backends;
package names vary.

- **iPhone/iPad:** unlock the device, connect it with a USB data cable, and accept
  **Trust This Computer** (enter the device passcode if requested). AFC can expose
  an app document-sharing view containing folders named after apps; this is not
  the photo library. For photos, install the gphoto2 backend and look for a
  separate camera/device entry with **DCIM**, if exposed by the phone. Accept any
  photo-access prompt. Photos stored only in iCloud may not be available over USB.
  Access depends on the iOS version and backend support; iOS does not expose
  unrestricted internal storage.
- **Android:** unlock the device, connect it with a USB data cable, and select
  **File transfer / Android Auto** or **MTP** in its USB preferences rather than
  charging-only mode. Accept any file-access prompt. Only storage exposed by the
  phone is available, not protected system files or private app data.

  On KDE Plasma, KDE's own MTP service (from `kio-extras`, used by Dolphin and the
  device notifier) competes with `gvfs-mtp` for the phone. Whichever claims it
  first wins: the other sees the phone with no storage, so Strata shows an empty
  volume, or Dolphin asks you to unlock the phone and tap **Allow**. To give the
  phone to Strata, unplug it, run `pkill -x kiod6` (`kiod5` on Plasma 5), and
  plug it back in unlocked. KDE restarts the service whenever a KDE app needs
  it, so repeat this if you used Dolphin before connecting the phone.
  **Settings → Diagnostics** warns when both are installed.

After installing a backend, fully quit Strata (all windows) and reopen it.
Reconnect the unlocked phone if necessary, then click its entry under **Devices**.
The camera/PTP entry opens a single **Photos** view: files appear progressively
from storage/date folders, prioritizing newer date-folder names, without requiring
you to open each folder. The backend may finish a folder's metadata before
returning its first batch. This is
a virtual listing of JPEG, HEIC/HEIF, MOV, MP4, and recognized camera RAW files,
not a reorganization of the phone. Sidecars such as `.AAE` and other formats are
hidden only from this Photos view; their originals remain untouched and visible
in normal folder browsing. Duplicate filenames remain
separate files with their original locations; use **Copy path** or **Properties**
to distinguish their sources. Preview, copy, and delete act on those originals.
This is the USB-exposed collection, not iOS's Albums hierarchy. The current
backend does not provide album membership for a reliable Albums/Camera Roll split.

Camera batches yield to interface input and redraws. In List view, file-type
grouping is applied after discovery finishes; the saved grouping setting is
preserved while the live listing stays ungrouped.

Discovery reads metadata, not every photo's contents. It skips symlinks, avoids
revisiting discovered directories, and preserves hidden-file filtering. Photos
keeps loading in batches until the whole exposed library is indexed, you navigate
away or refresh, or the device reports an error. There is no overall scan
deadline or fixed file, folder, or nesting limit. Refresh to rescan after a
phone-side change; not every device supports live change notifications.

Android MTP and iPhone app-document/AFC entries retain normal folder browsing.
For Android, open **Internal storage** (the label varies by device).

Still photos (including HEIC) and MOV/MP4 videos can use the preview pane or
<kbd>Space</kbd> quick preview directly from the phone, without a manual copy.
Strata first downloads a private temporary input for sandboxed decoding: up to
64 MiB in 30 seconds for images or 256 MiB in 60 seconds for videos, with at most
four staged inputs per process. Video playback waits for that download, then
reuses it for seeking and resizing. Closing or changing the preview cancels the
request; temporary files are removed after the player and its workers release
them. Remote PDFs, animated GIFs, audio, and other video formats still need a
local copy for preview. HEIC decoding requires an installed HEIC-capable image
decoder, such as ImageMagick with libheif. Camera/PTP thumbnails use small previews
provided by the camera, with bounded retrieval and sandboxed decoding; if the
camera cannot supply a thumbnail, Strata keeps the file icon instead of downloading
the original. Camera thumbnails are cached only in memory. See
[Remote previews](docs/preview-sandbox.md#remote-still-image-previews)
for cleanup, caching, and sandbox details.

If the phone is missing, check the backend package, try another data cable or USB
port, and confirm the trust/file-transfer setting. On Arch/Omarchy, `lsusb` (from
`usbutils`) can confirm USB detection, but detection alone does not establish file
access. For iPhone/iPad, also check `systemctl status usbmuxd.service` while the
phone is connected. If the newly installed backend still is not discovered after
restarting Strata, log out and back in to refresh the desktop's GVfs services.

#### iPhone appears but photo storage is empty

Some recent iPhones can expose an empty camera/PTP store with libgphoto2 2.5.34,
even when unlocked, trusted, and holding locally stored photos. This is a known
[upstream libgphoto2 issue](https://github.com/gphoto/libgphoto2/issues/1254), not
necessarily an empty photo library or a Strata display problem. The backend
mishandles the folder-parent information returned by these devices.

A read-only test with an iPhone reporting iOS 26.6.1 reproduced the problem:
unmodified libgphoto2 2.5.34 listed **0 folders**, while the same version with
[upstream fix `9f5d4f9`](https://github.com/gphoto/libgphoto2/commit/9f5d4f9ca0a7f58bac7987180a48154ea07c090f)
listed **117 folders**. Both builds were temporary, with their actual library
loading verified; no photos were downloaded or modified. This confirms folder
listing with the fix on that device, not complete transfer or Strata GUI coverage.

Use a distribution libgphoto2 update or backport containing that fix when
available. Merely reinstalling `gvfs-gphoto2` or restarting Strata will not fix an
affected libgphoto2 build. Strata's GVfs camera backend must load the corrected
library; setting library paths only for Strata may not affect the separately
launched GVfs process. This documentation change does **not** bundle or install
the fix. Avoid replacing system libraries manually; any locally built workaround
should be isolated and reversible.

### Network shares

Press <kbd>Ctrl</kbd>+<kbd>L</kbd>, enter an address such as `smb://server/share` or `sftp://user@host:2222/path`, and press <kbd>Enter</kbd>. Strata uses GIO/GVfs and prompts for credentials when required. Remote protocols need their GVfs backend installed; distributions split these up differently, so check yours for the SMB backend (`gvfs-smb` on Arch) and the SFTP backend (part of `gvfs` on Arch, `gvfs-backends` on Debian and Ubuntu).

Open folders on network shares, including these, are checked every few seconds while a window shows them, and re-read when another computer changed them; see [Changes made elsewhere](docs/filesystem-compatibility.md#changes-made-elsewhere).

SFTP accepts password and SSH-key authentication. Encrypted keys can be unlocked in your SSH agent; if GVfs asks Strata for a key passphrase or host-key decision, Strata presents the request without deciding for you. See [docs/remote-sftp.md](docs/remote-sftp.md), which also documents `scripts/sftp-fixture.sh`, a disposable OpenSSH server for testing.

## Custom actions and script authoring

Use **Settings → Actions → New action…** to create an action, or ask a coding
agent to create the files below. **Script → Library** supplies editable Python
and Bash recipes, sets their runtime, file filters, and **Whole selection / Per
item** mode, and never saves or executes them merely by selecting them.

For Python and Bash, the Script tab reports whether the interpreter can be found.
Python uses the script's shebang, or `python3` when there is none. Availability is checked
automatically when the dialog opens; reopen it after installing a runtime.
Bash and Command do not need Python. This checks
executable availability, **not** script syntax, safety, or dependencies such as
ImageMagick, FFmpeg, or ExifTool.

### Files and manifest

Actions are ordinary, user-owned folders under
`${XDG_CONFIG_HOME:-$HOME/.config}/strata/actions/`:

```text
strata/actions/log-selection/
├── action.toml
└── main.py
```

The folder name and manifest `id` must match. Use a new lowercase kebab-case id;
do not overwrite an existing action. Create directories with mode `0700` and
manifest/script files with mode `0600`. Entry points are regular files with plain
filenames, not symlinks or paths. Scripts need no executable bit: Strata invokes
the interpreter. Command actions have only a manifest. Import/export transfers
the manifest and declared entrypoint, not arbitrary helper files.

Example `action.toml`:

```toml
schema_version = 1
id = "log-selection"
name = "Log selected paths"
description = "Print the selection without modifying files"
icon = "terminal"
enabled = false
menu = "submenu"

[when]
kinds = ["file"]
min_items = 1

[run]
runtime = "python"
entrypoint = "main.py"
mode = "whole-selection"
on_error = "continue"
working_directory = "parent"
confirm = true
```

Keep generated actions disabled until the user reviews them. After hand-writing
files, restart Strata to reload them, then review/enable the action in Settings.
All selected entries must match the filters. Scripts are trusted local programs
with the user's permissions—**not sandboxed**.

### Python context

Put this in `main.py`; the helper is supplied by Strata at invocation time, with
no pip installation or copied helper module:

```python
#!/usr/bin/env python3
from pathlib import Path
from strata_actions import context

ctx = context()
for index, path in enumerate(ctx.paths, start=1):
    ctx.log(f"{index}: {Path(path).name}")
    ctx.progress(index, ctx.count, "Logging selected files")
```

- `ctx.paths`: absolute paths for this invocation; `ctx.paths_bytes()` preserves
  native filename bytes. `ctx.count` is their count, **not** the total per-item job
  size; `ctx.single` is the sole path or `None`.
- `ctx.position` / `ctx.total`: 1-based job position and job size in **Per item**
  mode, otherwise `None`. Whole-selection scripts enumerate `ctx.paths` themselves.
- `ctx.parent`: invoking folder. `ctx.directory`: stored action folder.
  `ctx.run_directory`: private temporary scratch, removed after the invocation.
- `ctx.log()`, `ctx.progress(processed, total=None, message=None)`, and
  `ctx.output(absolute_path)` report to Jobs. `output()` only reports a location;
  the script must create it. Import `require_tool(name)` from `strata_actions` to
  explain missing dependencies.

See the complete maintained [`context()` reference](data/actions/context-api.txt)
for identity, metadata, mode/source, and tool lookup. The **Batch rename** and
**Lowercase file names** recipes additionally pass a per-file naming context to
`new_name(context)`; its `filename` and 1-based `index` are recipe-specific, and
`context.batch` exposes the general Strata context.

### Bash and commands

For a Bash script, replace the manifest's `[run]` section with
`runtime = "bash"`, `entrypoint = "run.sh"`, and `mode = "whole-selection"`.
Example `run.sh`:

```bash
#!/usr/bin/env bash
set -euo pipefail
while IFS= read -r -d '' path; do
    printf 'Selected: %q\n' "$path"
done < "$STRATA_ACTION_PATHS"
```

`STRATA_ACTION_PATHS` is the **name of a file containing NUL-delimited paths**,
not a whitespace-separated list. Always quote paths; never use `for path in
$(cat ...)` or `eval`. `STRATA_ACTION_COUNT` is invocation-local;
`STRATA_ACTION_POSITION` is the 1-based per-item position. The invoking folder is
stored byte-exactly in the file named by `STRATA_ACTION_PARENT`. See the
[environment and progress protocol](docs/custom-actions.md#how-an-invocation-runs)
for context JSON, scratch paths, and progress reporting without Python.

For an installed executable, replace `[run]` with this—no script file:

```toml
[run]
runtime = "command"
program = "sha256sum"
args = ["--", "{paths}"]
mode = "whole-selection"
```

Programs receive direct argv, not shell source: pipes, redirection, `$VARIABLE`,
and globbing are not expanded. `{paths}` supplies separate arguments in whole
selection mode; `{path}` is for per-item mode; `{parent}` is the invoking folder.
Tokens must occupy an entire argument. Use Bash if shell syntax is needed.

**Choose the mode deliberately:** whole selection runs once with all paths;
per item runs once for each path, with Continue/Stop controlling subsequent
failures. Rename/lowercase/count-lines recipes use whole selection; conversions,
checksums, and EXIF stripping use per item. Library selections apply these modes
automatically, including when replacing a recipe from the other mode.

**Agent handoff checklist:** choose a fresh id, write the matching manifest and
entrypoint, declare required tools, preserve originals/refuse overwrites, and
keep the action disabled for review. Test on disposable copies through Strata's
context menu, then inspect Jobs output. Exit `0` means success; nonzero means
failure. Cancelling stops remaining work but does not undo filesystem changes.
The [full custom-actions guide](docs/custom-actions.md) documents validation,
matching rules, storage, examples, and limitations.

## Theming

Open **Settings → Appearance** from the gear menu or with <kbd>Ctrl</kbd>+<kbd>,</kbd>. Theme changes apply immediately across the interface.

Use **Search settings** to filter options across pages and navigate to the closest match, including keywords such as “font size.” In compact windows, the magnifying-glass button opens the search field. Clear the query or press <kbd>Esc</kbd> in the field to restore all settings.

![Strata Appearance settings showing theme following options, the current theme, and the searchable theme library](docs/assets/strata-themes.png)

### Follow system light/dark mode

Turn on **Follow system light/dark mode** under **Settings → Appearance** to have Strata switch between the light and dark versions of your chosen Strata theme when your desktop switches between light and dark mode. Only the light/dark preference is followed; Strata keeps its own theme palettes and does not adopt the desktop's GTK theme, icons, or accent color. Choose either version of a paired theme in the library to change the theme family without turning the switch off. Themes without a light/dark partner keep the same palette in both modes.

Strata reads the desktop light/dark preference from `org.freedesktop.portal.Settings` (`org.freedesktop.appearance`, `color-scheme`). The portal uses `1` for dark, `2` for light, and `0` for no preference. This supports KDE and Flatpak. When the portal reports no preference or is unavailable, Strata follows GNOME's `org.gnome.desktop.interface` `color-scheme` key; `prefer-dark` selects dark and `default`, `prefer-light`, and unexpected values select light. When the switch is on, Strata waits up to 500 ms for the first portal result before applying its initial palette. It uses the GSettings result (or light) only after a portal failure, no preference, or timeout; after a timeout it updates when the portal reply arrives. The switch is hidden and a saved system selection is repaired only after neither source is available. **Follow system light/dark mode** and **Follow Omarchy** are exclusive; enabling either one turns the other off.

### Follow Omarchy Quattro

On **Omarchy Quattro**, turn on **Follow Omarchy** under **Settings → Appearance**. Strata maps the active Omarchy palette to its semantic colors, monitors the current theme, and updates live whenever Omarchy's theme changes.

This integration supports Omarchy Quattro only. The switch is hidden when Strata cannot find a valid Quattro current-theme state; legacy Omarchy theme layouts are not supported.

### Bundled themes

Choose any of the 95 included themes from the searchable, light/dark-filterable library under **Settings → Appearance**, including Tokyo Night (the default), Azure Glow, Catppuccin, Everforest, Rosé Pine, Gruvbox, Nord, Dracula, Solarized, and Omarchy Light. Selecting a bundled theme turns off Omarchy following and keeps that theme active across restarts. While **Follow system light/dark mode** is on, the selection instead changes the theme family Strata switches within.

### Custom themes

Select **Add a theme**, enter a name, and choose the semantic colors for the background, surfaces, text, accent, danger, muted and highlighted elements, borders, and dimmed text. Strata previews edits live and saves completed themes under **Your themes**.

Custom themes are stored as shareable TOML files in `~/.config/strata/themes/`. A selected theme is re-read when its file changes on disk, so a generator that rewrites it in place (for example a [matugen](https://github.com/InioX/matugen) template following the wallpaper) updates open windows without a restart. See [Themes](docs/themes.md) for the schema, file location, live updates, and Omarchy color mapping.

To go beyond the color tokens, an optional `~/.config/strata/style.css` is loaded above Strata's own stylesheets and re-read when it changes; see [User stylesheet](docs/themes.md#user-stylesheet).

## Release channels

> [!NOTE]
> Release channels and **Install a specific release** deliver the **official** Strata
> releases from lgse/strata. Installing one replaces this fork's build. This fork has
> no releases or channels of its own; update it by rebuilding, as described in
> [Update](#update).

Strata defaults to the **Stable** channel: only final tagged releases are ever offered, and a Stable install never receives, sees, or is notified about a prerelease.

Strata checks for updates automatically by default: each time it starts, once its first window opens, and again when a window or **Settings → Updates** opens if no check has completed in the last 24 hours. Turning **Check for updates automatically** back on, or changing the channel, also runs a check. The check asks the update source for your installation: GitHub for release binaries and source builds, the AUR for AUR packages, the Omarchy package repository for Omarchy packages, or the local pacman sync database for other pacman packages. When a newer version is found, Strata also fetches that release's details from GitHub. Turning the switch off stops the automatic checks, discards the result of a check still running, hides any update notice already shown, and locks the channel. **Check now** still checks on demand, and the Updates page's release-notes card asks GitHub for the current version's notes when they are not already cached. Strata never downloads or installs an update without your confirmation.

To try upcoming changes early, choose a channel in **Settings → Updates** (the channel is locked while automatic checks are off). **Preview** receives curated alpha, beta, and release-candidate builds but excludes nightlies. **Nightly** receives every recognised prerelease, including daily development builds. The update dialog and release notes always identify the exact build kind.

When a prerelease installation selects **Stable**, the Updates card immediately offers the newest stable release as the channel target—even when that requires a semantic downgrade—and labels the action **Return to stable**. Preview and Nightly selections use the same card for ordinary forward updates, so channel changes never create a separate competing rollback card.

To roll back or switch to a particular build, open **Install a specific release** in **Settings → Updates**. It lists the releases your channel offers, newest first, whether older or newer than the installed one: Stable lists stable releases, Preview adds alpha, beta, and release-candidate builds, and Nightly lists everything. Strata asks GitHub for the list only when the menu opens, and lists only releases with a signed update manifest for your architecture. Choosing one opens the same verified install dialog as an update. Package-managed installations do not show the list, since their package manager decides the version.

See [Releasing](docs/releasing.md) for the tag grammar these channels rely on and, for maintainers, how a release candidate is cut and promoted.

## Under the hood

### Why search stays fast

Strata walks and indexes the selected directory tree on a background thread, never on GTK's UI thread. Results appear progressively during that walk. Names and relative paths are normalized once as index entries are created, and each query maintains only the best **100** fuzzy-ranked matches instead of sending an unbounded result set to the interface.

Each walk stops at the saved time and entry limits (10 s and 200,000 entries by default, each adjustable up to no limit) and reports partial results. Rapid keystrokes are coalesced to the newest query. During indexing, result publication is throttled to 50 ms intervals; the UI consumes those bounded updates on its own timed loop, keeping rendering aligned with responsive frame-sized work. Exact and contiguous matches, word/path boundaries, and names rank ahead of loose path subsequences.

The deliberate tradeoff: this is fast **filename and path** search, not file-content or metadata search.

### How previews contain untrusted parsers

Files shown while browsing are untrusted. Image, camera RAW, PDF, thumbnail, and media parsing therefore runs out of process through **Bubblewrap**, not inside the main Strata process. Each short-lived helper receives namespace isolation, a minimal read-only runtime, exactly one canonicalized input file, private output and temporary directories, no network, and no capabilities. Memory, CPU/wall time, input, file, and parent-side output limits bound the work.

Only media helpers may receive allowlisted GPU render devices, and only for accelerated decoding; image, PDF, and thumbnail helpers receive no device mounts. Images are normalized to bounded PNG images. Media arrives incrementally as validated raw RGBA frames and fixed-format PCM: GTK presents textures and GStreamer outputs raw audio, without opening the original file or decoding a compressed clip. Four media sessions per process, bounded queues, and paused-worker cleanup limit concurrent work. Cancellation or timeout kills the process group and Bubblewrap PID namespace, tearing down descendants. Missing isolation, crashes, malformed output, timeouts, and permission failures all fail closed to a normal icon or **Preview unavailable**—Strata never silently retries an untrusted native parser without the sandbox.

Plain-text and source previews are different: they stay in process because they do not invoke a native format parser, and reads are capped at 1 MiB. See [Preview sandbox](docs/preview-sandbox.md) for provider ordering, exact mounts, formats, and resource budgets. For settings that further reduce exposure, such as software-only video decoding, see [Hardened configuration](docs/hardened-configuration.md).

## Technical specifications

| Area | Details |
| --- | --- |
| Platform | 64-bit glibc-based Linux; designed for Omarchy and Wayland. GTK may use another backend supplied by the host, but Wayland is the primary display stack. Official release binaries need glibc 2.39+; this fork is built on your system. |
| Architectures | `x86_64` and `aarch64`, built from source |
| UI and runtime | Rust 2024, GTK 4.12+, GIO/GLib, Cairo, GtkSourceView 5, Poppler GLib, GDK Pixbuf, GStreamer, and Fontconfig |
| Filesystems | Native Linux paths (including non-UTF-8 names) and GIO/GVfs locations; remote protocol availability depends on installed GVfs backends |
| Preview boundary | Bubblewrap is mandatory for native parser-backed previews; helpers have no network and fail closed. Plain text is read in process with a 1 MiB cap. |
| Optional preview tools | `ffmpegthumbnailer`/`ffmpeg` for video; ImageMagick, classic `dcraw`, and LibRaw `simple_dcraw` expand camera RAW support; `squashfs-tools` (`unsquashfs`) extracts icons embedded in AppImages |
| Hardware acceleration | Media-only VA-API or Vulkan decoding with software fallback; GPU and codec support depend on host drivers |
| Scale targets | Virtualized browser models and bounded asynchronous updates are tested with deterministic directories up to 100,000 entries |
| Packaging | This fork: built from source with `install.sh`, Cargo or Nix; no release archives. Official releases: dynamically linked archive with a signed update manifest, SHA-256 digest, GitHub build-provenance attestation, and `SOURCE_COMMIT` |

## Development and documentation

Build requirements are the latest stable Rust toolchain, a C toolchain, `pkg-config`, GTK 4.12+, GtkSourceView 5, Poppler GLib, Fontconfig, and GStreamer 1.20+ (including its app/base development libraries). [mise](https://mise.jdx.dev) pins that toolchain locally (`mise install`). On Arch:

```bash
sudo pacman -S --needed base-devel bubblewrap ffmpeg ffmpegthumbnailer fontconfig \
  gstreamer gst-libav gst-plugins-base gst-plugins-good gtk4 gtksourceview5 gvfs poppler-glib
mise run install-local    # build an optimized release and install it for your user
mise run start-dev        # rebuild and restart as files change
mise run dev              # build and launch the main app once
mise run chooser-dev      # build and open an isolated Save chooser with choices
mise run check            # format, compile, Clippy, tests, and policy checks
```

`start-dev`, `dev`, and `chooser-dev` make unoptimized debug builds for development, which
scroll and load noticeably slower. Use `install-local` for everyday use. A debug build reports
its version with a `+debug` suffix in `strata --version` and in Settings → About.

### Builds without RAR support

The default `rar` Cargo feature embeds RARLAB's non-free UnRAR implementation.
Builds of this fork, official releases and AUR builds retain RAR extraction and CBR
cover previews.
Packagers who cannot distribute UnRAR can exclude it with:

```bash
cargo build --locked --release --no-default-features
```

This build does not compile or link `unrar` or `unrar_sys`. RAR files are not
offered for extraction, and CBR cover previews report that RAR support is disabled.
ZIP, 7z, TAR, CBZ, and EPUB support is unchanged. Use `--features rar` to opt back
in; `--all-features` also enables UnRAR. The source lockfile and license notices
still describe the optional dependency; they do not mean it is linked into the
feature-disabled binary. See [third-party notices](THIRD_PARTY_LICENSES.md#unrar).

### Builds without the in-app updater

Distributions that ship Strata updates through their own packages build with
`STRATA_UPDATER=disabled`, which removes update checks, installs, and the
Updates settings page. See [distribution packaging](docs/packaging.md#building-without-the-in-app-updater).

### Builds with package-provided integrations

Store-based packages such as Nix build with `STRATA_INTEGRATIONS=package`, which
turns off the runtime installers for the file chooser portal, the FileManager1
service, the Omarchy shortcuts, and the udiskie unlock hook so the package can
ship those files itself. See [distribution packaging](docs/packaging.md#building-with-package-provided-integrations).

To contribute to Strata itself, read [CONTRIBUTING.md](CONTRIBUTING.md) and open pull requests on the [official repository](https://github.com/lgse/strata). This fork's changes are made directly on its `main`. Deeper references:

- [Keyboard navigation](docs/keyboard-navigation.md)
- [Keymap](docs/keymap.md)
- [Preview panel and column layout](docs/preview-panel-layout.md)
- [10xer mode](docs/10xer-mode.md)
- [Architecture principles](docs/architecture.md)
- [Filesystem compatibility](docs/filesystem-compatibility.md)
- [Administrator access](docs/administrator-access.md)
- [Preview sandbox](docs/preview-sandbox.md)
- [Hardened configuration](docs/hardened-configuration.md)
- [Performance baseline](docs/performance-baseline.md)
- [Custom actions](docs/custom-actions.md)
- [External file providers and bundled integrations](docs/file-providers.md)
- [Themes and Omarchy integration](docs/themes.md)
- [Unsafe code policy](docs/unsafe-code.md)
- [Releasing](docs/releasing.md)

## Credits

Strata is created and maintained by **LGSE Ltd.** and the
[Strata contributors](https://github.com/lgse/strata/graphs/contributors) at
[github.com/lgse/strata](https://github.com/lgse/strata). This fork exists only
because of their work: the file manager, its design, its documentation and nearly all
of its code are theirs. To support the project, use and star the
[official repository](https://github.com/lgse/strata).

<a href="https://github.com/lgse/strata/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=lgse/strata" alt="Avatars of Strata contributors linked to the contributors graph">
</a>

Fork changes are by [parasite6](https://github.com/parasite6).

## License

Strata is free software licensed under the **[MIT License](LICENSE)**, copyright
LGSE Ltd. This fork keeps that license and copyright notice unchanged, and its own
changes are released under the same license. Bundled fonts, icons, and other
third-party components retain their own notices in
[THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).

"Strata" and its logo belong to their respective owners. Their use here identifies the
project this fork is based on and does not imply endorsement.
