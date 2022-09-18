FROM rustembedded/cross:arm-unknown-linux-gnueabihf

RUN dpkg --add-architecture arm32 && apt-get update 
RUN apt-get install --assume-yes libsqlite3-dev