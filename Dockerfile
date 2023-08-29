FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/Chicago
RUN apt-get update \
    && apt-get install -y build-essential curl pandoc texlive-xetex librsvg2-bin texlive-science texlive \
        texlive-fonts-recommended texlive-fonts-extra texlive-full software-properties-common git \
    && add-apt-repository ppa:deadsnakes/ppa \
    && apt-get install -y python3.9 python3-pip python3.9-distutils
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y \
    && PATH="/root/.cargo/bin:${PATH}" \
    && rustup install nightly-2022-02-24 \
    && rustup default nightly-2022-02-24 \
    && rustup component add rust-src \
    && rustup component add rustc-dev \
    && rustup component add llvm-tools-preview \
    && cargo install xargo \
    && printf "[build]\nrustflags = ['-L', '${HOME}/.rustup/toolchains/nightly-2021-12-14-x86_64-unknown-linux-gnu/lib']\n" \
        > $HOME/.cargo/config.toml \
    && python3.9 -m pip install --upgrade setuptools \
    && python3.9 -m pip install --upgrade pip \
    && python3.9 -m pip install --upgrade distlib \
    && python3.9 -m pip install toml pypandoc matplotlib plotly kaleido pandas \
    && update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.9 10 \
    && update-alternatives --set python3 /usr/bin/python3.9 \
    && mkdir -p /root/.ssh/

RUN apt install -y pkg-config libudev-dev libssl-dev

ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /vrust
COPY . .

# WORKDIR /vrust/mirav
# RUN cargo clean \
#     && cargo build
# WORKDIR /vrust

# ENTRYPOINT ["python3.9", "/vrust/examples/run.py", "-v", "/vrust/target/debug/vrust", "-o", "./SmartV_Report_Generator/"]
