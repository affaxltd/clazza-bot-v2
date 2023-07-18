################
##### Runtime
FROM archlinux:latest 

COPY ./target/release/bot /usr/local/bin

CMD ["/usr/local/bin/bot"]
