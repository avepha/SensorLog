FROM rustembedded/cross:arm-unknown-linux-gnueabihf
ENV DEBIAN_FRONTEND=noninteractive
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV RPI_TOOLS=/rpi_tools
ENV MACHINE=armv6
ENV ARCH=armv6
ENV CC=gcc
ENV RPI=1

RUN dpkg --add-architecture armhf
RUN apt-get update &&\
    apt-get install -y pkg-config libudev-dev:armhf libsqlite3-dev:armhf
