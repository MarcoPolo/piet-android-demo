# Piet Android Demo

This is a Demo app that uses
[piet-android](https://github.com/marcopolo/piet) to draw a line with Rust. It also draws to a bitmap and saves it in a cache file to inspect later.

## Running

The easiest approach is probably to just install Android Studio and open this folder. Then connect your phone and hit run.

You can also try using Gradle from the command line with `./gradlew installDebug` and opening the app on the Android device.

## Checking out the png
You'll need root since the file is saved in the application's cache folder. Then run `adb pull /data/user/0/io.marcopolo.pietdemo/cache/temp-image.png` to fetch the image.
