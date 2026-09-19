FROM mcr.microsoft.com/devcontainers/rust:2.1-trixie

RUN apt-get update && apt-get install -y ripgrep

RUN mkdir -p /home/vscode/persistent-config && chown vscode:vscode /home/vscode/ /home/vscode/persistent-config

USER vscode
WORKDIR /home/vscode

# Target for the web build, see README ("Build (web)") and the CI wasm job.
RUN rustup target add wasm32-unknown-unknown

# Keep this in sync with the wasm-bindgen crate version in Cargo.lock.
RUN cargo install --locked wasm-bindgen-cli --version 0.2.128

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/home/vscode/persistent-config/.bash_history" \
  && echo "$SNIPPET" >> "/home/vscode/.bashrc"

WORKDIR /workspaces/snake
