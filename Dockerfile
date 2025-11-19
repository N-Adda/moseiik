FROM rust:latest

WORKDIR /app

# on copie les fichiers de dépendances pour les builder en cache
COPY Cargo.toml Cargo.lock ./
# On crée un fichier main.rs factice pour compiler les dépendances en cache
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
# on supprime le code compilé pour ne garder que le cache des dépendances
RUN rm -rf src target/release/.fingerprint target/release/build target/release/deps/moseiik*

# copie du code source 
COPY . /app
# on compile le code complet
RUN cargo build --release
RUN cargo test --release --no-run

ENTRYPOINT [ "cargo", "test", "--release", "--" ]