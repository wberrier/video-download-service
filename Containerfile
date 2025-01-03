FROM debian:12

ARG VDS_SRC=/usr/src/video-download-service
ARG VDS_USER=vds
ARG VDS_HOME_DIR=/var/cache/vds

# Base install/config
RUN \
	apt-get update \
	&& apt-get install -y \
	systemd systemd-sysv udev \
	vim \
	iputils-ping procps iproute2 \
	&& apt-get clean \
	&& systemctl enable systemd-networkd \
	&& systemctl mask getty@tty1.service \
	&& echo "NAutoVTs=0" >> "/etc/systemd/logind.conf"

# Install dependencies
RUN \
	apt-get update \
	&& apt-get install -y \
	cargo \
	curl \
	ffmpeg libavcodec-extra \
	&& apt-get clean

# Configure dhcp networking
RUN \
	echo "[Match]\nType=ether\n\n[Network]\nDHCP=yes" > "/etc/systemd/network/20-wired.network"

# Install yt-dlp
RUN \
	mkdir -p /usr/local/bin \
	&& curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp \
	&& chmod a+rx /usr/local/bin/yt-dlp

# Configure user
RUN \
	useradd --create-home --home-dir $VDS_HOME_DIR --user-group $VDS_USER \
	&& usermod --lock $VDS_USER

# Copy source into container (but specifically not user config, to be able to iterate)
RUN mkdir -p $VDS_SRC
COPY ./    $VDS_SRC/

# Build, install, and cleanup
RUN cd $VDS_SRC \
	&& cargo build --release \
	&& cp target/release/video-download-service /usr/local/bin \
	&& cp files/usr/lib/systemd/system/video-download-service.service /usr/lib/systemd/system \
	&& systemctl enable video-download-service \
	&& rm -Rf target

# Run the services via systemd
# NOTE: works in podman, not sure about docker...
CMD [ "/sbin/init" ]
