FROM rust:1.84

WORKDIR /usr/src/img_brd
COPY . .

RUN cargo install --path .

CMD ["axum_ssr_image_board"]