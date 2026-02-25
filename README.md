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

## WASM

Для запуска web `trunk serve --open --port=8081`

### Настройка

- Настройка env: создать в по пути `crates/wasm` файл `.env` на основе `.env.example` с корректными данными
