# Публикация в Яндекс Играх

## Что уже подготовлено

- русские title/description/keywords, `manifest.json` и help/about;
- Яндекс SDK: fullscreen-реклама после поражения, rewarded-реклама для добровольной разблокировки Skjold и баннер только на экранах входа/возрождения;
- русский загрузочный экран с текущими игровыми fallback-ассетами;
- `publication.json` и скрипт сборки архива.

## Обязательный серверный шаг

Клиент Kodiak получает `/system.json` и определяет WebSocket-хост по URL ответа. Поэтому статический архив из Консоли сам по себе не подключится к игровому Rust-серверу.

Перед загрузкой выберите один вариант:

1. настроить reverse proxy, чтобы `/system.json`, `/translation.json` и WebSocket шли через тот же origin;
2. для стенда открыть пакет с `?yandex=1&server_origin=https%3A%2F%2FYOUR-SERVER`; сервер должен разрешать CORS для origin Яндекс Игр. Для production используйте закреплённый origin, а не произвольный query-параметр.

## Сборка

```bash
./yandex/package.sh
```

Архив появится в `yandex/yandex-games.zip`. Команда требует Rust `nightly-2024-04-20` и `trunk 0.21.7`; их можно установить командами `make rustup` и `make trunk`.

## Чек-лист Консоли

1. Создать черновик игры, выбрать русский язык, альбомную ориентацию и desktop/mobile.
2. Загрузить `yandex/yandex-games.zip`, указать URL/прокси игрового сервера и включить монетизацию Яндекс Игр.
3. Проверить гостевой вход без обязательной авторизации, звук при потере фокуса, мобильный touch и подключение к серверу.
4. Проверить fullscreen после поражения, баннер только вне активного боя и rewarded с понятным бонусом.
5. Заполнить возрастные/правовые поля и отправить на модерацию.

## Ассеты

GPT Image 2 промпты сохранены в `yandex/image-prompts.jsonl`, но текущий локальный `OPENAI_API_KEY` отклонён API (401). После обновления ключа выполните:

```bash
python3 /Users/vlad/.codex/skills/.system/imagegen/scripts/image_gen.py generate-batch \
  --input yandex/image-prompts.jsonl --out-dir client/data --concurrency 3
./yandex/package.sh
```

До этого архив использует существующие корабельные изображения из `client/data/`; новые icon/cover/loading-файлы нельзя считать готовыми.
