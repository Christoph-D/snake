FROM mcr.microsoft.com/devcontainers/rust:2.0-trixie

RUN apt-get update && apt-get install -y ripgrep

RUN mkdir -p /home/vscode/persistent-config && chown vscode:vscode /home/vscode/ /home/vscode/persistent-config

USER vscode
WORKDIR /home/vscode

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/home/vscode/persistent-config/.bash_history" \
  && echo "$SNIPPET" >> "/home/vscode/.bashrc"

WORKDIR /workspaces/snake
