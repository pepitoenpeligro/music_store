FROM rust

WORKDIR ./
COPY . .

RUN cargo install --path ./
RUN cargo build

EXPOSE 8000

CMD ["cargo", "run", "music-store"]
