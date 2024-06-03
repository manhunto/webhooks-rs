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
- [ ] SDK
- [ ] CLI
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

## 👨‍💻 Development

### Prerequisites

- **[just](https://github.com/casey/just)** - optional, if you want to run raw commands
- **[docker with docker-compose](https://www.docker.com/products/docker-desktop/)** - optional, if you want to set up
  the environment on your own

## 🤝 Contribution

If you want to contribute to the growth of this project, please follow
the [conventional commits](https://www.conventionalcommits.org/) in your pull requests.
