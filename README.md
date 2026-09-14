# Флот Онлайн

[![Сборка](https://github.com/SamosGames/flot-online/actions/workflows/build.yml/badge.svg)](https://github.com/SamosGames/flot-online/actions/workflows/build.yml)

«Флот Онлайн» — многопользовательский морской бой от SamosGames. Управляйте кораблём, собирайте добычу, улучшайтесь и топите соперников.

## Сборка

1. Установите `rustup` ([инструкция](https://rustup.rs/)).
2. Установите `gmake` и `gcc`, если их ещё нет в системе.
3. Установите Trunk, Rust Nightly и цель WebAssembly:

```console
make rustup
make trunk
```

4. Соберите клиент:

```console
cd client
make release
```

5. Соберите и запустите сервер:

```console
cd server
make run_release
```

6. Откройте `https://localhost:8443/`.

## Разработка

Данные кораблей, оружия, самолётов, препятствий и добычи находятся в `common/src/entity/_type.rs`. Для публикации в Яндекс Играх используйте [инструкции пакета](yandex/README.md): там описаны сборка ZIP, подключение игрового сервера и рекламные события.

## Авторство

Проект основан на открытом коде Softbear Studios и распространяется по лицензии AGPL-3.0-or-later. Сторонние материалы перечислены в разделе «Об игре».
