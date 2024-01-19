set dotenv-load

build-release:
  @echo "Building release..."
  @LEPTOS_OUTPUT_NAME="my-leptos-app-$(tr -dc a-z0-9 </dev/urandom | head -c 10)" cargo leptos build --release --precompress
