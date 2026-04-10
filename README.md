# Passwords — Menubar App

Минималистичный менеджер паролей в виде приложения из строки меню. Быстрый доступ к учётным данным одним кликом.

## Возможности

- Список паролей в виде компактных карточек
- Мгновенный поиск по названию сервиса и домену
- Фильтры: Все, Частые, Недавние, 2FA, Проблемы безопасности
- Раскрытие карточки — masked пароль, кнопки копирования, открытие сайта
- Идентификационные тайлы с монограммами сервисов
- Статус-бейджи: 2FA, Passkey, Weak, Reused
- Счётчик на иконке трея
- Глобальная горячая клавиша (`Cmd/Ctrl+Shift+Space`)
- Поддержка светлой и тёмной тем
- Закрытие при клике вне окна

## Стек

- **Бэкенд**: Rust + Tauri 2
- **Фронтенд**: Vue 3 + TypeScript
- **Сборка**: Vite 8
- **Линтинг**: oxc (фронтенд), Clippy (бэкенд)

## Запуск в режиме разработки

```bash
bun install
bun tauri dev
```

## Сборка для установки

### macOS (.app / .dmg)

```bash
bun tauri build
```

Результат:

- `src-tauri/target/release/bundle/macos/Passwords.app` — готовое приложение
- `src-tauri/target/release/bundle/dmg/Passwords_0.1.0_aarch64.dmg` — DMG для распространения

### Windows (.exe / .msi)

```bash
bun tauri build
```

Результат:

- `src-tauri/target/release/bundle/msi/Passwords_0.1.0_x64_en-US.msi`
- `src-tauri/target/release/bundle/nsis/Passwords_0.1.0_x64-setup.exe`

### Linux (.deb / .AppImage)

```bash
bun tauri build
```

Результат:

- `src-tauri/target/release/bundle/deb/passwords_0.1.0_amd64.deb`
- `src-tauri/target/release/bundle/appimage/passwords_0.1.0_amd64.AppImage`

> **Примечание:** Для сборки под конкретную платформу используйте флаг `--target`:
>
> ```bash
> bun tauri build --target universal-apple-darwin    # macOS Universal
> bun tauri build --target aarch64-apple-darwin      # macOS Apple Silicon
> bun tauri build --target x86_64-pc-windows-msvc    # Windows x64
> ```

## Линтинг

```bash
# Фронтенд (oxc)
bun run lint

# Бэкенд (clippy)
cd src-tauri && cargo clippy
```

## Структура проекта

```
src-tauri/src/
├── main.rs      # Точка входа, инициализация
├── state.rs     # Состояние приложения (AppState)
├── tray.rs      # Tray иконка, бейджи, меню
├── window.rs    # Управление окнами
└── commands.rs  # Tauri команды

src/
├── types/
│   └── password.ts        # Типы для паролей
├── data/
│   └── mockPasswords.ts   # Моковые данные
├── composables/
│   ├── useCounter.ts      # Composable для счётчика
│   └── usePasswords.ts    # Composable для паролей
├── components/
│   ├── HeaderBar.vue      # Заголовок
│   ├── SearchField.vue    # Поле поиска
│   ├── FilterRow.vue      # Фильтры-чипы
│   └── PasswordCard.vue   # Карточка пароля
├── App.vue                # Главный компонент
├── main.ts                # Точка входа Vue
└── style.css              # Глобальные стили с CSS переменными
```
