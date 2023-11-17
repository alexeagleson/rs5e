FROM rust:1.72.1-slim-buster as builder

ENV project=rs5e-app

### RS5E components

WORKDIR /usr/src/${project}/crates/rs5e-components
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-components/Cargo.toml .

### RS5E concepts

WORKDIR /usr/src/${project}/crates/rs5e-concepts
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-concepts/Cargo.toml .

### RS5E DATA-MODEL

WORKDIR /usr/src/${project}/crates/rs5e-schema
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-schema/Cargo.toml .

### RS5E DICE

WORKDIR /usr/src/${project}/crates/rs5e-dice
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-dice/Cargo.toml .

### RS5E ENTITIES

WORKDIR /usr/src/${project}/crates/rs5e-entities
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-entities/Cargo.toml .

### RS5E log

WORKDIR /usr/src/${project}/crates/rs5e-log
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-log/Cargo.toml .

### RS5E macro-derive

WORKDIR /usr/src/${project}/crates/rs5e-macro-derive
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-macro-derive/Cargo.toml .

### RS5E systems

WORKDIR /usr/src/${project}/crates/rs5e-systems
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/lib.rs
COPY ./crates/rs5e-systems/Cargo.toml .

### MAIN APP

WORKDIR /usr/src/${project}/

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create a dummy release build that builds all the app's real dependencies
RUN cargo build --release

# Remove dummy files
RUN rm ./**/*.rs

# Copy actual source code
COPY ./src ./src
COPY ./data ./data
COPY ./crates/rs5e-components/src ./crates/rs5e-components/src
COPY ./crates/rs5e-concepts/src ./crates/rs5e-concepts/src
COPY ./crates/rs5e-dice/src ./crates/rs5e-dice/src
COPY ./crates/rs5e-entities/src ./crates/rs5e-entities/src
COPY ./crates/rs5e-log/src ./crates/rs5e-log/src
COPY ./crates/rs5e-macro-derive/src ./crates/rs5e-macro-derive/src
COPY ./crates/rs5e-schema/src ./crates/rs5e-schema/src
COPY ./crates/rs5e-systems/src ./crates/rs5e-systems/src

# Update the timestamps on the main source files so they recompile
# and don't use any caching from the dummy builds
RUN touch ./src/main.rs
RUN touch ./crates/rs5e-components/src/lib.rs
RUN touch ./crates/rs5e-concepts/src/lib.rs
RUN touch ./crates/rs5e-schema/src/lib.rs
RUN touch ./crates/rs5e-dice/src/lib.rs
RUN touch ./crates/rs5e-entities/src/lib.rs
RUN touch ./crates/rs5e-log/src/lib.rs
RUN touch ./crates/rs5e-macro-derive/src/lib.rs
RUN touch ./crates/rs5e-systems/src/lib.rs

# Build the real app
RUN cargo build --release

### CLIENT STUFF

FROM node:20.8-buster-slim as server

ENV project=rs5e-app

# ENV project=rs5e-app

# SHELL [ "/bin/bash", "-l", "-c" ]

# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash

# ENV NVM_DIR "$HOME/.nvm"
# ENV NODE_VERSION 20
# # RUN source ~/.nvm/nvm.sh && nvm install 20 && nvm use 20

# # # Install Node (to build the Vite app)
# # Install Node (to build the Vite app)
# RUN nvm install 20
# RUN nvm use 20

WORKDIR /usr/src/${project}

COPY ./client/package.json ./client/package.json
COPY ./client/package-lock.json ./client/package-lock.json

WORKDIR /usr/src/${project}/client

# Install client dependencies
RUN npm install

COPY ./client .

# Do a production build of the Vite client
RUN npm run build

# Back to project root dir
WORKDIR /usr/src/${project}

# Copy the server build artifact from the build stage
COPY --from=builder /usr/src/${project}/target/release/${project} .

# Copy the raw JSON DnD data over to the runner image
COPY --from=builder /usr/src/${project}/data ./data

# Copy the production build of the Vite app
# COPY --from=builder /usr/src/${project}/client/dist ./client/dist

# ### BACK TO SERVER STUFF

# FROM builder

# # Copy the binary to root
# RUN cp ./target/release/${project} ./${project}

# ### CLIENT

# COPY ./static ./static

### LAUNCH

EXPOSE 8080

# CMD ["sleep","infinity"]
CMD ["./rs5e-app", "0.0.0.0:8080"]


