FROM rust:1.67

WORKDIR /usr/src/meals-service

COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD ["meals-service"]