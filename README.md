> [!CAUTION]
> The project is not finished, it is not stable and it is constantly being developed.

# webhooks-rs

<div align="center">
    <img src="assets/logo.jpeg" width="350">
</div>

<p align="center">
    <a href="https://codecov.io/gh/manhunto/webhooks-rs"><img src="https://codecov.io/gh/manhunto/webhooks-rs/graph/badge.svg?token=C10FE6520S"/></a>
    <img src="https://github.com/manhunto/webhooks-rs/actions/workflows/rust.yml/badge.svg?branch=master" alt="build"/>
    <img src="https://img.shields.io/github/contributors/manhunto/webhooks-rs" alt="contributors"/>
    <img src="https://img.shields.io/github/commit-activity/m/manhunto/webhooks-rs" alt="commit-activity"/>

</p>

## ℹ️ About

**webhooks-rs** is a project for sending webhooks using the http protocol. The main goals and objectives are to create
an application that is high-performing, configurable and scalable.

>
> \[!NOTE]
>
> This project takes part and was created thanks to the [100 Commits](https://100commitow.pl/) challenge and is my first
> significant project written in Rust.

### MVP features

- [x] Retry policy for failed messages
- [x] Endpoint can be disabled manually
- [x] Circuit breaker
- [x] Persistence
- [x] SDK Beta
- [x] CLI Beta
- [ ] Dockerized
- [ ] Documentation
- [x] Integration tests
- [ ] Error handling and validation

### Roadmap

- [ ] Rate-limit
- [ ] Auth
- [ ] Signed webhooks - server can verify that message was sent from valid server
- [ ] Distributed architecture
- [ ] Data retention
- [ ] Logging and monitoring

## Domain explanation

**Application** - Is a container that groups endpoints. In a multi-tenant architecture, it can be a separate tenant.
Each application can have a separate configuration and secrets (in progress...).

**Endpoint** - This is the url of the server to which messages are sent. Each endpoint can be deactivated individually -
either manually or automatically by the circuit breaker.

**Event** - This is an event that originated in your system. The event has a topic and a payload. For now, it only
supports JSON payload.

**Message** - In a nutshell, it can be said to be an event for a given endpoint. A given event can be distributed to
several endpoints.

**Attempt** - This is a log of attempts to deliver a particular message. A given message may have multiple delivery
attempts (e.g. endpoint is temporarily unavailable and message had to be retried by retry policy).

## ⚙️ How to use?

### Server

Before run environment by using `just init`. This command run a docker and execute migrations. Server is split into two
parts - server and dispatcher. Run `just rs` and `just rd`.

Server has rest api interface. Example commands you can find in `server/server.http`. Please familiarise oneself
with [Domain Explanation](#domain-explanation)

### SDK

> \[!IMPORTANT]
>
> SKD requires running server and dispatcher. See [Server](#server) section.

You can find an example of the use of the sdk in the [examples/src/producer-server.rs](examples/src/producer-server.rs)

### Cli

> \[!IMPORTANT]
>
> Cli requires running server and dispatcher. See [Server](#server) section.

To explore all possibilities run `cargo run --package=cli`. Cli is divided by resources sections.

#### Create application

```shell
$ cargo run --package=cli application create "example application"
App app_2hV5JuBgjMAQlDNNbepHTFnkicy with name 'example application' has been created
```

#### Create endpoint

To create an endpoint in a recently created application

```shell
$ cargo run --package=cli endpoint create app_2hV5JuBgjMAQlDNNbepHTFnkicy http://localhost:8090/ contact.created,contact.updated
Endpoint ep_2hV67JEIXUvFCN4bv43TUXVmX0s has been created
```

#### Create event

```shell
$ cargo run --package=cli event create app_2hV5JuBgjMAQlDNNbepHTFnkicy contact.created '{"foo":"bar"}'
Event evt_2hV6UoIY9p6YnLmiawSvh4nh4Uf has been created
```

## 👨‍💻 Development

### Prerequisites

- **[just](https://github.com/casey/just)** - optional, if you want to run raw commands
- **[docker with docker-compose](https://www.docker.com/products/docker-desktop/)** - optional, if you want to set up
  the environment on your own

## 🤝 Contribution

If you want to contribute to the growth of this project, please follow
the [conventional commits](https://www.conventionalcommits.org/) in your pull requests.
