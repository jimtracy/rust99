FROM rust
  
#EXPOSE 8000

RUN mkdir -p /root/rust99

COPY ./rust99 /root/rust99

WORKDIR /root/rust99
RUN cargo build

VOLUME ["/root/rust99"]

#CMD ["cargo","test","--all"]
CMD ["tail","-f","/dev/null"]
