# Проектная работа rust 3

Егоров Дмитрий

## Настройка pre-commit

Включение pre-commit `pre-commit install`

Локальный запуск `pre-commit run --verbose --all-files`

## GitHub

Для корректной сборки sqlx в github необходимо выполнить `cargo sqlx prepare` из `crates/server`

## Server

Запуск командой `cargo run --bin server`

### Настройка

- Установка protobuf: `brew install protobuf`
- Настройка env: создать в по пути `crates/server` файл `.env` на основе `.env.example` с корректными данными
- Для первоначальной миграции надо запустить bin файл из `crates/server` командой `cargo run --bin migrate`

## CLI

Для просмотра доступных команд запустите `cargo run help`

Примеры команд:

```bash
# http
cargo run http http://127.0.0.1:8080 register --email="email" --password="password" --username="username"
cargo run http http://127.0.0.1:8080 login --email="email" --password="password" # сохранение токена в файл
# grpc
cargo run grpc http://127.0.0.1:50051 get-post-list
cargo run grpc http://127.0.0.1:50051 create-post --title="title" --content="content"
```

## WASM

Для запуска web `trunk serve --open --port=8081`

Для редактирования/удаления постов необходимо нажать на пост в таблице

### Настройка

- Настройка env: создать в по пути `crates/wasm` файл `.env` на основе `.env.example` с корректными данными
