ARG CROSS_BASE_IMAGE
FROM $CROSS_BASE_IMAGE

ENV RUBY_VERSION=3.1.2

RUN apt-get update -yq && apt-get install -yq build-essential gnupg2 openssl ruby ruby-dev
RUN curl -sSL https://rvm.io/pkuczynski.asc | gpg2 --import -
RUN curl -L https://get.rvm.io | bash -s stable
RUN /bin/bash -l -c "rvm requirements"
RUN /bin/bash -l -c "rvm install 3.1.2"
RUN /bin/bash -l -c "rvm install 3.0.4"
RUN /bin/bash -l -c "rvm install 2.7.6"
RUN /bin/bash -l -c "rvm install 2.6.10"

COPY build/entrypoint.sh /
RUN chmod +x "/entrypoint.sh"

ENTRYPOINT ["/entrypoint.sh"]
