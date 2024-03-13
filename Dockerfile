FROM debian:12-slim

WORKDIR /app

COPY target/release/mini-server /app/mini-server

COPY www/* /app/www

RUN chmod 755 mini-server

EXPOSE 8080

CMD ["./app/mini-server"]
