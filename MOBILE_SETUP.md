# 📱 Мобильная сборка (Android & iOS)

Этот проект поддерживает сборку для **Android** и **iOS** с помощью Tauri v2.

## ✅ Выполненная настройка

- [x] Установлены Rust targets для мобильных платформ
- [x] Добавлены мобильные зависимости в `Cargo.toml`
- [x] Создана мобильная конфигурация `tauri.mobile.conf.json`
- [x] Инициализирован Android проект
- [x] Инициализирован iOS проект
- [x] Добавлен `lib.rs` с `mobile_entry_point` для мобильных платформ
- [x] Настроена условная компиляция для tray-функций

## 🔧 Структура мобильной сборки

### Файлы

- **`src/lib.rs`** - точка входа для мобильных платформ с `#[tauri::mobile_entry_point]`
- **`src/main.rs`** - точка входа для десктопа (macOS)
- **`commands.rs`** - использует `#[cfg(not(mobile))]` для tray-специфичного кода

### Условная компиляция

Мобильный код помечен `#[cfg(mobile)]`, десктопный - `#[cfg(not(mobile))]`.

## 📋 Требования

### Для Android

1. **Android SDK** (уже установлен ✓)
2. **Android NDK** (уже установлен ✓)
3. Переменные окружения (опционально):
   ```bash
   export ANDROID_HOME=$HOME/Library/Android/sdk
   export NDK_HOME=$HOME/Library/Android/sdk/ndk/<version>
   ```

### Для iOS

1. **Xcode** (уже установлен ✓)
2. **CocoaPods** (уже установлен ✓)
3. **Rust targets** (уже установлены ✓):
   - `aarch64-apple-ios`
   - `x86_64-apple-ios`
   - `aarch64-apple-ios-sim`

### Подписание кода (iOS)

Для сборки iOS приложения требуется сертификат разработки Apple:

1. Откройте Xcode → Preferences → Accounts
2. Войдите в свой Apple ID
3. Установите `APPLE_DEVELOPMENT_TEAM` переменную окружения или добавьте в `tauri.mobile.conf.json`:
   ```json
   {
     "bundle": {
       "iOS": {
         "developmentTeam": "YOUR_TEAM_ID"
       }
     }
   }
   ```

## 🚀 Команды для сборки

### Android

```bash
# Сборка для Android (включает Android Studio проект)
pnpm tauri android build --config src-tauri/tauri.mobile.conf.json

# Запуск в режиме разработки
pnpm tauri android dev --config src-tauri/tauri.mobile.conf.json

# Открыть проект в Android Studio
open src-tauri/gen/android/
```

#### Целевые платформы Android:
- `aarch64-linux-android` (ARM64 - большинство современных устройств)
- `armv7-linux-androideabi` (ARM32 - старые устройства)
- `x86_64-linux-android` (эмулятор)
- `i686-linux-android` (эмулятор 32-bit)

### iOS

```bash
# Сборка для iOS (включает Xcode проект)
pnpm tauri ios build --config src-tauri/tauri.mobile.conf.json

# Запуск в режиме разработки (симулятор)
pnpm tauri ios dev --config src-tauri/tauri.mobile.conf.json

# Открыть проект в Xcode
open src-tauri/gen/apple/tauri-menubar.xcodeproj
```

#### Целевые платформы iOS:
- `aarch64-apple-ios` (физические устройства)
- `aarch64-apple-ios-sim` (симулятор M1/M2)
- `x86_64-apple-ios` (симулятор Intel)

## ⚠️ Важные ограничения

### macOS-специфичные функции (не работают на мобильных)

Основная конфигурация `tauri.conf.json` использует функции, доступные только на macOS:

- ❌ `macOSPrivateApi`
- ❌ `windowEffects: ["popover"]`
- ❌ `alwaysOnTop: true`
- ❌ `skipTaskbar: true`
- ❌ `transparent: true`

Для мобильных платформ используется отдельная конфигурация `tauri.mobile.conf.json`, которая не включает эти функции.

### Global Shortcut

Плагин `tauri-plugin-global-shortcut` может работать иначе на мобильных платформах. На Android/iOS глобальные горячие клавиши не поддерживаются так же, как на десктопе.

## 📱 Тестирование на симуляторах

### Android Emulator

1. Откройте Android Studio
2. Tools → Device Manager → Create Virtual Device
3. Выберите устройство и образ системы
4. Запустите:
   ```bash
   pnpm tauri android dev --config src-tauri/tauri.mobile.conf.json
   ```

### iOS Simulator

```bash
# Запуск в симуляторе iPhone
pnpm tauri ios dev --config src-tauri/tauri.mobile.conf.json --target x86_64-apple-ios

# Или для M1/M2
pnpm tauri ios dev --config src-tauri/tauri.mobile.conf.json --target aarch64-apple-ios-sim
```

## 🛠 Известные проблемы

### Android

- **Проблема**: Ошибка компиляции Rust для Android
  **Решение**: Убедитесь, что `cargo-ndk` установлен:
  ```bash
  cargo install cargo-ndk
  ```

### iOS

- **Проблема**: Ошибка подписи кода
  **Решение**: Установите `APPLE_DEVELOPMENT_TEAM`:
  ```bash
  export APPLE_DEVELOPMENT_TEAM="YOUR_TEAM_ID"
  ```

## 📚 Дополнительные ресурсы

- [Tauri Mobile Guide](https://tauri.app/v1/guides/building/mobile/)
- [Tauri Android](https://tauri.app/develop/supported-platforms/android/)
- [Tauri iOS](https://tauri.app/develop/supported-platforms/ios/)

## 🔄 Обновление мобильных проектов

При изменении конфигурации Tauri может потребоваться перегенерировать мобильные проекты:

```bash
# Android
pnpm tauri android init --config src-tauri/tauri.mobile.conf.json

# iOS
pnpm tauri ios init --config src-tauri/tauri.mobile.conf.json
```

⚠️ **Внимание**: Это перезапишет изменения в сгенерированных файлах. Не редактируйте файлы в `src-tauri/gen/android/` и `src-tauri/gen/apple/` напрямую.
