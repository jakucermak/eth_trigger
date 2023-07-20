FROM rust:1.67

RUN apt update && apt install -y libpcap-dev \
                                iptraf-ng \
                                iproute2 \
                                procps

RUN useradd -ms /bin/bash pi

# USER pi

COPY ./config.toml /home/pi/.config/eth_trigger/

WORKDIR /app
COPY Cargo.toml ./


COPY ./src ./src

ENTRYPOINT /bin/bash
