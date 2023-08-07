# Ethernet Triger using raspberry pi

## Goal

Main goal is to achieve near real-time generating PWM signal on Raspberry Pi GPIO ports. Triggering an action is based on incoming packet filtered by BPF [IBM doc](https://www.ibm.com/docs/en/qsip/7.4?topic=queries-berkeley-packet-filters) or
[BPF syntax](https://biot.com/capstats/bpf.html) and after that checking byte specific at position specified in [configuration file](./config.toml).

## Instalation
### Requirements
For installing you need sudo privileges, Raspberry Pi.

Packages needed:
1. libpcap-dev
`sudo apt install libpcap-dev`

### Installation process
#### Debug build
In project root folder run
`cargo build`
#### Release build
`cargo build --release`

## Run trigger
1. Copy [configuration file](./config.toml) to root's config folder. If not exists run `sudo mkdir -p /root/.config/eth_trigger/`. To copy config run following `sudo cp ./config.toml /root/.config/eth_trigger/config.toml`

2. Edit config to your needs as sudo using your favourite text editor `vi` `vim` `nano` `etc..`


# TODO:
- [ ] Create build script
