FROM rust:alpine3.14

WORKDIR /home
COPY . .

CMD ["sh", "./init-docker.bash"]
