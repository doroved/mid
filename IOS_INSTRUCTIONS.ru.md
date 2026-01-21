# Инструкция по использованию mid на iOS

<img src="mid_example_ios.png" width="300" alt="iOS Example App" />

Эта библиотека позволяет генерировать уникальный идентификатор устройства (Machine ID) на iOS, используя **Keychain**.

**Особенности реализации:**
*   Идентификатор генерируется как `SHA256(Nanoid + Timestamp)` и сохраняется в Keychain.
*   При переустановке приложения ID сохраняется (на реальных устройствах).
*   Для разделения ID между разными приложениями используется `Service Name`.

## 1. Быстрый запуск

В репозитории подготовлен пример готового приложения.

1.  Откройте папку **`ios_example`** в **Xcode**.
2.  Выберите симулятор или реальное устройство.
3.  Нажмите **Run (▶)**.

## 2. Ручная интеграция в ваш проект

Если вы создаете новый проект или интегрируете библиотеку в существующий:

### Шаг 1: Подготовка фреймворка

Если у вас нет файла `Mid.xcframework`, соберите его:

```bash
chmod +x build_ios.sh
./build_ios.sh
```

После сборки в корне появится папка `Mid.xcframework`.

### Шаг 2: Добавление в Xcode

1.  Откройте ваш проект в Xcode.
2.  Перетащите файл **`Mid.xcframework`** в навигатор проекта (левая панель).
    *   В появившемся окне поставьте галочку **"Copy items if needed"**.
3.  Нажмите на настройки вашего Таргета (Target) -> вкладка **General**.
4.  В разделе **Frameworks, Libraries, and Embedded Content** найдите `Mid.xcframework`.
5.  Установите опцию **"Do Not Embed"**.
    *   *Важно: Библиотека статическая, поэтому Embed не нужен.*

### Шаг 3: Добавление кода (Wrapper)

Для работы Swift с Rust-библиотекой нужна небольшая обертка. Создайте файл `MidManager.swift` и добавьте туда следующий код:

```swift
import Foundation
import Mid // Если Xcode не видит модуль, убедитесь, что фреймворк добавлен в Target

class MidManager {
    /// Получить хеш устройства (Machine ID)
    /// - Parameter serviceName: Уникальное имя сервиса для Keychain (например, "com.myapp.mid").
    static func get(serviceName: String) -> String? {
        guard let cServiceName = serviceName.cString(using: .utf8) else { return nil }
        
        // Вызов C-функции из библиотеки Rust
        guard let resultPtr = mid_get(cServiceName) else { return nil }
        
        let mid = String(cString: resultPtr)
        mid_free_string(resultPtr) // Освобождение памяти Rust
        
        return mid
    }
}
```

### Шаг 4: Использование во View

Пример использования в SwiftUI:

```swift
import SwiftUI

struct ContentView: View {
    @State private var machineID: String = "..."
    let serviceName = "io.github.doroved.mid.example" // Уникальный ID вашего сервиса

    var body: some View {
        VStack {
            Text("Machine ID:").font(.headline)
            Text(machineID).padding()
        }
        .onAppear {
            if let mid = MidManager.get(serviceName: serviceName) {
                machineID = mid
            }
        }
    }
}
```

## 3. Тестирование

*   **Симулятор:** При удалении приложения с симулятора Keychain обычно сбрасывается. При повторной установке ID может измениться.
*   **Реальное устройство:** Это самый надежный тест. Keychain сохраняется даже после удаления приложения. Если вы удалите приложение и установите его снова (с тем же `serviceName`), библиотека вернет **тот же самый ID**.
