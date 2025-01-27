# Use the official Ubuntu base image
FROM ubuntu:22.04

# Set non-interactive mode for installing packages
ENV DEBIAN_FRONTEND=noninteractive

# Install necessary packages (you can add more based on your needs)
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    wget \
    vim \
    pkg-config \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Create a volume for persistence
VOLUME /usr/src

# Set /usr/src as the working directory
WORKDIR /usr/src

# Copy everything from the current directory into /usr/src in the container
COPY . /usr/src/

# Keep the container running
CMD ["tail", "-f", "/dev/null"]
