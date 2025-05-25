# cosmic-connect-applet

An attempt to provide KDE Connect functionality in a cosmic applet

## Inspiration from other KDE Connect projects

- [valent](https://github.com/andyholmes/valent/)
- [gs-connect](https://github.com/GSConnect/gnome-shell-extension-gsconnect)
- [kde-connect](https://invent.kde.org/network/kdeconnect-kde)

## Rough Todo

If something is not completed or market as in progress and you'd like to help out, please head over to Issues and/or make a pull request

- [ ] Read and send SMS
- [ ] Send clipboard
- [ ] Share files/folder
- [ ] Browse device files
- [ ] Remote input
- [ ] Media control
- [ ] Ring lost device
- [ ] Presenter
- [ ] Lock device
- [ ] Battery status
- [ ] Connectivity status
- [ ] Control volume
- [ ] Keep awake
- [ ] Use as a monitor
- [ ] Deamon for the non-interactive, chronjob tasks like detecting notifications on the device

## Rough functionality

- All actions should be queryable, similar to cosmic settings where known actions of unknown locations can easily be found
- UI should be clean and aligned with cosmic design principles
- All functionality should be contained within the applet: no secondary app for settings, sms services, etc. except for obvious cases like file viewing (using cosmic files)

## Install

To install your the applet, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install your applet:

```sh
just build-release
sudo just install
```

Then add it to your panel in the Cosmic Settings app