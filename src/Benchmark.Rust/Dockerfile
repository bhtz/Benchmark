# Use the official Rust base image with the specified version
FROM rust:1.75

# Set the working directory to /usr/src/app
WORKDIR /usr/src/app

# Copy the local Cargo.toml and Cargo.lock files into the container
COPY Cargo.toml Cargo.lock ./

# Copy the entire project source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Expose the port that Actix will run on
EXPOSE 8080

# Set the environment variable for Actix to run in production mode
ENV ACTIX_ENV=production

# Run the application
CMD ["target/release/benchmark-rust"]
