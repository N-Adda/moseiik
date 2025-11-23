# Dockerfile pour builder et tester l'application Rust Moseiik
# On utilise l'image officielle Rust comme base qui compile sur les deux architectures amd64 et arm64
# Les commandes pour build et run pour l'arch amd64 est :
    #docker build --platform linux/amd64 -t moseiik:amd64 .
    #docker run --platform linux/amd64 moseiik:amd64
# Les commandes pour build et run pour l'arch arm64 est :
    #docker build --platform linux/arm64 -t moseiik:arm64 .
    #docker run --platform linux/arm64 moseiik:arm64
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