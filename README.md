# clashup

> The manager of clash

## Guide

### Inital

Inital config and download clash.

```shell
$ clashup init --mode proxy
```

### Start a daemon

Start daemon.
```shell
$ clashup daemon
```

Start daemon with update of subscription

```shell
$ clashup daemon -s https://example.com?token=0xxxx&flag=clash
```

### Update subscription

Use this command to update subscription.

```shell
$ clashup config https://example.com?token=0xxxx&flag=clash
```

## Command

### init

Inital clash config. This command will do these step:

1. download and unpack clash
2. create clash config based on `mode`
3. download Country.mmdb
4. download controller panel

#### clashup mode

Now clashup support the following modes:

- proxy: Only as a standalone proxy, open mixer, socks5 and http port. System or application need set to these port 
- openwrt-tun: Create tun interface on openwrt, and set wan as outbound interface.

### daemon

Start clash daemon or create systemd daemon

- `--install-systemd-unit`: (root needed) Use this flag to install systemd unit in system.

### subscription

Manage subscriptions. update, add new or install systemd unit.

- `--install-systemd-unit`: (root needed) Use this flag to install subscription update systemd unit in system.
- `--add-subscription <name>`: Add subscribe.
- `--del-subscription <name>`: Del subscription.
- `--set-subscription <name>`: Update subscription url.
- `--update-subscription <name>`: Update a special subscription.
- `--update-all`: Update all subscription.
- `--list-subscription`: List all subscriptions.

### cli

A command line client for clash.

- `--endpoint`: A clash controller endpoint.

#### log

Display a real-time log.

#### config

Display and set basic config.

Display all config and version.

```shell
$ clash cli config
```

Set mode. role, global or direct. Or other config item.

- `--rule`: Rule mode.
- `--global`: Global mode.
- `--direct`: Direct mode.
- `--http-port <number>`: Set http proxy port.
- `--socks5-port <number>`: Set proxy5 proxy port.

#### proxies

Check proxies and select proxy.

Display all proxies.

```shell
$ clashup proxies
```

These is flags and arguments.

- `--show-delay`: Show delay on all proxies.
- `--select <index>`: Select a proxies.
- `<index>`: Show special index of proxy.

#### Rules

Display all rules

```shell
$ clashup rules
```

Add rules

```shell
$ clashup rules --add <set> --type <ip> [192.168.0.0/24]
```

Sets:

- Reject
- Direct
- Proxy

