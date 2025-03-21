FROM mcr.microsoft.com/devcontainers/rust:1.0-bookworm

RUN apt-get update && apt-get install -y vim --no-install-recommends && rm -rf /var/lib/apt/lists/*

# alias と GPG_TTY の設定
RUN echo "alias c='cargo'" >> /etc/bash.bashrc && \
    echo "alias g='git'" >> /etc/bash.bashrc && \
    echo "alias m='makers'" >> /etc/bash.bashrc && \
    echo "export GPG_TTY=\$(tty)" >> /etc/bash.bashrc

# vscode ユーザーで cargo binstall
ENV CARGO_MAKE_VERSION=0.37.23
USER vscode
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y cargo-make@${CARGO_MAKE_VERSION} cargo-watch grcov
