FROM rust:1.77

WORKDIR /var/opt/roydy
RUN cargo install dioxus-cli@^0.5
RUN rustup component add rustfmt

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs
RUN npm install -g npm
RUN rustup target add wasm32-unknown-unknown

COPY ./roydy/package.json ./
RUN npm install
RUN npm install -g tailwindcss@3.4.3

