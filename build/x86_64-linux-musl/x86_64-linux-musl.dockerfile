FROM messense/rust-musl-cross:x86_64-musl

ENV RUBY_TARGET="x86_64-linux-musl" \
  RUST_TARGET="x86_64-unknown-linux-musl" \
  RUSTUP_DEFAULT_TOOLCHAIN="stable" \
  RUSTUP_HOME="/root/.rustup" \
  CARGO_HOME="/root/.cargo" \
  CARGO_BUILD_TARGET="x86_64-unknown-linux-musl" \
  PKG_CONFIG_ALLOW_CROSS="1" \
  PATH="/root/.cargo/bin:$PATH" \
  LIBCLANG_PATH="/usr/lib/llvm-14/lib" \
  CC_x86_64_unknown_linux_musl="x86_64-unknown-linux-musl-gcc" \
  CXX_x86_64_unknown_linux_musl="x86_64-unknown-linux-musl-g++" \
  AR_x86_64_unknown_linux_musl="x86_64-unknown-linux-musl-ar" \
  BINDGEN_EXTRA_CLANG_ARGS_x86_64_unknown_linux_musl="--sysroot=/usr -I/usr/include/x86_64-linux-musl" \
  CMAKE_x86_64_unknown_linux_musl="cmake"

# Add dependencies
RUN apt-get -y update && \
    apt-get install -y \
    build-essential \
    cmake \
    curl \
    dirmngr \
    git-core \
    gcc-multilib \
    gnupg2 \
    libreadline-dev \
    libssl-dev \
    musl \
    musl-dev \
    musl-tools \
    pkg-config \
    sudo \
    unzip \
    wget \
    xz-utils \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# Add "rvm" as system group, to avoid conflicts with host GIDs typically starting with 1000
RUN groupadd -r rvm && useradd -r -g rvm -G sudo -p "" --create-home rvm

# Make sure rvm and later settings are available in interactive and non-interactive shells
RUN echo "source /etc/profile.d/rvm.sh" >> /etc/rubybashrc && \
    echo "source /etc/rubybashrc" >> /etc/bashrc && \
    echo "source /etc/rubybashrc" >> /etc/bash.bashrc
ENV BASH_ENV /etc/rubybashrc

USER rvm

RUN mkdir ~/.gnupg && \
    chmod 700 ~/.gnupg && \
    echo "disable-ipv6" >> ~/.gnupg/dirmngr.conf

# install rvm, RVM 1.26.0+ has signed releases, source rvm for usage outside of package scripts
RUN gpg --keyserver hkp://keyserver.ubuntu.com --recv-keys 409B6B1796C275462A1703113804BB82D39DC0E3 7D2BAF1CF37B13E2069D6956105BD0E739499BDB && \
    (curl -L http://get.rvm.io | sudo bash) && \
    bash -c " \
        source /etc/rubybashrc && \
        rvm autolibs disable && \
        rvmsudo rvm cleanup all "

# Import patch files for ruby and gems
COPY patches /home/rvm/patches/

# install rubies and fix permissions on
ENV RVM_RUBIES 3.1.0
RUN bash -c " \
    export CFLAGS='-s -O3 -fno-fast-math -fPIC' && \
    for v in ${RVM_RUBIES} ; do \
        rvm install \$v --patch \$(echo ~/patches/ruby-\$v/* | tr ' ' ','); \
    done && \
    rvm cleanup all && \
    find /usr/local/rvm -type d -print0 | sudo xargs -0 chmod g+sw "

# Install rake-compiler and typical gems in all Rubies
# do not generate documentation for gems
RUN echo "gem: --no-ri --no-rdoc" >> ~/.gemrc && \
    bash -c " \
        rvm all do gem update --system --no-document && \
        rvm all do gem install --no-document bundler 'bundler:~>1.16' 'rake-compiler:1.1.6' hoe mini_portile rubygems-tasks mini_portile2 && \
        find /usr/local/rvm -type d -print0 | sudo xargs -0 chmod g+sw "

# Install rake-compiler's cross rubies in global dir instead of /root
RUN sudo mkdir -p /usr/local/rake-compiler && \
    sudo chown rvm.rvm /usr/local/rake-compiler && \
    ln -s /usr/local/rake-compiler ~/.rake-compiler

USER root

RUN bash -c " \
    rvm alias create default 3.1.0 && \
    rvm use default "

# Patch rake-compiler to build and install static libraries for Linux rubies
USER rvm
COPY patches2 /home/rvm/patches/
RUN bash -c " \
    for v in ${RVM_RUBIES} ; do \
      cd /usr/local/rvm/gems/ruby-\$v/gems/rake-compiler-1.1.6 && \
      echo applying patches to ruby-\$v /home/rvm/patches/rake-compiler-1.1.6/*.patch && \
      ( git apply /home/rvm/patches/rake-compiler-1.1.6/*.patch || true ) \
    done "

# Build xruby versions with ruby2_keywords in parallel using ruby-3.x
ENV XRUBIES "3.1.0:3.0.0:2.7.0"
# Build xruby versions in parallel
# Then cleanup all build artifacts
RUN bash -c " \
    rvm use 3.1.0 && \
    export CFLAGS='-O1 -fno-omit-frame-pointer -fno-fast-math -fstack-protector-strong' && \
    export LDFLAGS='-pipe' && \
    export LIBS='' && \
    export MAKE='make V=1 -j`nproc`' && \
    rake-compiler cross-ruby VERSION=$XRUBIES HOST=x86_64-linux && \
    rm -rf ~/.rake-compiler/builds ~/.rake-compiler/sources && \
    find /usr/local/rvm -type d -print0 | sudo xargs -0 chmod g+sw "

# Avoid linking against libruby shared object.
# See also https://github.com/rake-compiler/rake-compiler-dock/issues/13
RUN find /usr/local/rake-compiler/ruby/*linux*/ -name libruby.so | xargs rm
RUN find /usr/local/rake-compiler/ruby/*linux*/ -name libruby-static.a | while read f ; do cp $f `echo $f | sed s/-static//` ; done
RUN find /usr/local/rake-compiler/ruby/*linux*/ -name libruby.a | while read f ; do ar t $f | xargs ar d $f ; done
RUN #find /usr/local/rake-compiler/ruby/*linux*/ -name mkmf.rb | while read f ; do sed -i ':a;N;$!ba;s/TRY_LINK = [^\n]*\n[^\n]*\n[^\n]*LOCAL_LIBS)/& -lruby-static -lpthread -lrt -ldl <% if platform=~/x86/ %> -lcrypt <% end %>/' $f ; done

USER root

# Fix paths in rake-compiler/config.yml
RUN sed -i -- "s:/root/.rake-compiler:/usr/local/rake-compiler:g" /usr/local/rake-compiler/config.yml

# Make `musl` an actual target in rake-compiler
RUN sed -i -- "s:rbconfig-x86_64-linux-gnu-:rbconfig-x86_64-linux-musl-:g" /usr/local/rake-compiler/config.yml

# Install SIGINT forwarder
COPY sigfw.c /root/
RUN /usr/bin/gcc /root/sigfw.c -o /usr/bin/sigfw

# Install user mapper
COPY runas /bin/runas
COPY rcd-env.sh /etc/profile.d/
RUN echo "source /etc/profile.d/rcd-env.sh" >> /etc/rubybashrc

# Install sudoers configuration
COPY sudoers /etc/sudoers.d/rake-compiler-dock

ENV RUBY_CC_VERSION 3.1.0:3.0.0:2.7.0

CMD bash

COPY rubybashrc.sh /rubybashrc.sh
RUN /rubybashrc.sh

RUN chmod -R ugo+rwX /root/
