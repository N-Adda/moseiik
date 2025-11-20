FROM rust:latest

WORKDIR /app

# on copie les fichiers de dépendances pour les builder en cache
COPY Cargo.toml Cargo.lock ./
# On crée un fichier main.rs factice pour compiler les dépendances en cache
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
# on supprime le code compilé pour ne garder que le cache des dépendances
RUN rm -rf src target/release/.fingerprint target/release/build target/release/deps/moseiik*

RUN wget https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip \
    && unzip moseiik_test_images.zip -d assets \
    && rm moseiik_test_images.zip

# copie du code source 
COPY . .
# on compile le code complet
RUN cargo build --release

ENTRYPOINT [ "cargo", "test", "--release", "--" ]