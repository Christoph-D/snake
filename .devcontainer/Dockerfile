FROM mcr.microsoft.com/devcontainers/rust:2.0-trixie

RUN apt-get update && apt-get install -y ripgrep

RUN mkdir -p /home/vscode/persistent-config && chown vscode:vscode /home/vscode/ /home/vscode/persistent-config

USER vscode
WORKDIR /home/vscode

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/home/vscode/persistent-config/.bash_history" \
  && echo "$SNIPPET" >> "/home/vscode/.bashrc"

ENV OPENCODE_CONFIG=/home/vscode/persistent-config/opencode.json
RUN curl -fsSL https://opencode.ai/install | bash
ENV PATH="/home/vscode/.opencode/bin:$PATH"

WORKDIR /workspaces/snake
