### DEV target
FROM rust:slim-buster as dev

ARG USER="runner"

ENV USER=${USER}
ENV HOMEDIR="/home/${USER}"
ENV BIN_DIR="${HOMEDIR}/bin" \
    WORKDIR="${HOMEDIR}/app"

ENV CARGO_HOME="${HOMEDIR}/.cargo/"

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
    gdb \
    gcc \
    make \
    openssl \
    pkg-config \
    libssl-dev

RUN cargo install cargo-nextest --locked
RUN cargo install cargo-watch

# ENV RUST_LOG=debug
# ENV RUST_BACKTRACE=full

RUN useradd ${USER} -m
WORKDIR ${WORKDIR}
ENV PATH=${BIN_DIR}:$PATH

COPY . ${WORKDIR}

RUN chown ${USER}:${USER} -R ${HOMEDIR}

USER ${USER}
RUN cargo fetch

expose 8088

CMD ["make", "dev"]
