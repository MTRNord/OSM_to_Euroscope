# How to use

You will need RUST (See https://rustup.rs/ on how to install it)

After installing Rust you can use `cargo install --git https://github.com/MTRNord/OSM_to_Euroscope.git` to install the Application.

```shell script
$ OSM_to_Euroscope taxiways -a=<ICAO ID OF YOUR AIRPORT>
```

This command will generate the GROUND layout of the taxiways to a file named `<ICAO ID OF YOUR AIRPORT>.ese` inside the out folder.


## WARNING!! THE QUALITY OF THE RESULT HIGHLY DEPENDS ON THE QUALITY OF THE MAPPED DATA!
