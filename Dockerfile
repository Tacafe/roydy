FROM rust:1.77

WORKDIR /var/opt/roydy

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs
RUN npm install -g npm

COPY ./roydy/ ./
RUN npm install
RUN npm install -g tailwindcss@3.4.3

RUN rustup component add rustfmt
RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli
RUN cargo add dioxus-web
