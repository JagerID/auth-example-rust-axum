# IDK Backend

## Запуск

1. Установите [rust](https://www.rust-lang.org/tools/install)
2. Запустите postgres-db: `sudo docker compose -f ./compose-dev.yml up -d`
3. Установите sqlx-cli: `cargo install sqlx-cli`
4. Запустите миграции: `sqlx migrate run`
5. Запустите бекенд: `cargo run`