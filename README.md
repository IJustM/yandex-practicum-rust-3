# Проектная работа rust 3

Егоров Дмитрий

## Настройка проекта

- Установка protobuf: `brew install protobuf`
- Настройка env: создать в по пути `crates/server` файл `.env` на основе `.env.example` с корректными данными
- Для первоначальной миграции надо запустить bin файл из `crates/server` командой `cargo run --bin migrate`

## Настройка pre-commit

Включение pre-commit `pre-commit install`

Локальный запуск `pre-commit run --verbose --all-files`
