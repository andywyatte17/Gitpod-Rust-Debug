FROM gitpod/workspace-full

USER gitpod

RUN sudo apt-get -q update \
    && sudo apt-get install -yq \
        lldb-9 \
        python3-lldb-9 \
        rust-lldb \
        lldb \
    && sudo rm -rf /var/lib/apt/lists/*
