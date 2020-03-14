####################################################################
#
####################################################################
FROM rustlang/rust:nightly AS build

ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl
ENV DEBIAN_FRONTEND=noninteractive
ENV PKG_CONFIG_ALLOW_CROSS=1

ENV INSTALL_PATH /app/current
WORKDIR $INSTALL_PATH

COPY ./docker /docker
RUN /docker/base_packages.sh

COPY . .
RUN /docker/build.sh
RUN chmod +x /app/current/email_hooks

####################################################################
# Use scratch so we can get an itty-bitty-teeny-tiny image.
# This requires us to use musl when building the application.
####################################################################
FROM scratch AS final-destination

EXPOSE 9000

# COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=build /app/current/email_hooks /app/current/email_hooks
COPY --from=build /app/current/default-config.yaml /app/current/default-config.yaml
COPY --from=build /dumb-init /dumb-init

ENTRYPOINT ["/dumb-init", "--rewrite", "15:3", "--"]
CMD ["/app/current/email_hooks", "-vvv", "-c", "/app/current/default-config.yaml"]
