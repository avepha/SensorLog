FROM rustembedded/cross:armv7-unknown-linux-gnueabihf
ENV DEBIAN_FRONTEND=noninteractive
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV RPI_TOOLS=/rpi_tools

RUN dpkg --add-architecture armhf
RUN apt-get update &&\
    apt-get install -y pkg-config libudev-dev:armhf libsqlite3-dev:armhf
