# Архитектурный аудит telegram-ui

## Контекст
Проект представляет собой UI-библиотеку для Telegram-клиента, написанную на Rust.

---

## ❌ Критические проблемы

### 1. Отсутствие SDK-кранта

**Проблема:** В проекте нет отдельного `telegram-sdk` crate, содержащего бизнес-логику, модели данных и сетевой слой.

**Почему критично:**
- Нарушен принцип разделения ответственности
- Невозможно тестировать бизнес-логику независимо от UI
- Нет возможности использовать SDK в других контекстах (mobile, desktop)
- Все зависимости направлены неправильно (UI = SDK)

**Рекомендация:** Создать отдельный `telegram-sdk` crate с:
```
telegram-sdk/
  src/
    models/      # User, Chat, Message, etc.
    api/         # HTTP clients, request/response types
    network/     # WebSocket, connection management
    services/    # Business logic
```

---

### 2. UI-библиотека вместо SDK + UI

**Проблема:** Текущая структура `telegram-ui-core` содержит только UI-компоненты, но нет бизнес-логики. Однако это не SDK — это UI-абстракция над компонентами.

**Почему критично:**
- Нет ясного контракта между SDK и UI
- Невозможно мокать внешние зависимости
- Нет event-driven архитектуры

---

## ⚠️ Замечания

### 1. Отсутствие state management

**Проблема:** В `webapp.rs` и `context/` нет явного управления состоянием.

**Последствия:**
- UI не может масштабироваться
- Нет реактивности
- Состояние может быть раскидано по глобальным переменным

**Рекомендация:** Реализовать pattern:
```rust
// telegram-sdk/src/state/mod.rs
pub struct AppState {
    user: Option<User>,
    chats: Vec<Chat>,
    messages: HashMap<ChatId, Vec<Message>>,
    // ...
}

pub trait StateManager {
    fn get(&self) -> &AppState;
    fn update(&mut self, action: AppStateAction);
}
```

---

### 2. Отсутствие event-driven архитектуры

**Проблема:** Нет каналов для событий (новые сообщения, статус отправки, изменения).

**Рекомендация:** Добавить:
```rust
// telegram-sdk/src/events/mod.rs
pub enum AppEvent {
    NewMessage(Message),
    MessageDelivered(MessageId),
    UserTyping(UserId),
    ConnectionStatusChanged(ConnectionState),
}

pub struct EventManager {
    sender: mpsc::Sender<AppEvent>,
    // broadcast channels for different event types
}
```

---

### 3. Отсутствие trait-абстракций для инверсии зависимостей

**Проблема:** UI-код напрямую зависит от конкретных реализаций, а не от trait'ов.

**Последствия:**
- Нельзя замокать SDK для тестирования UI
- Нет гибкости при замене реализаций

**Рекомендация:**
```rust
// telegram-sdk/src/api/mod.rs
pub trait ApiClient {
    fn get_messages(&self, chat_id: ChatId) -> Result<Vec<Message>, ApiError>;
    fn send_message(&self, msg: SendMessageRequest) -> Result<(), ApiError>;
}

pub struct HttpClient {
    client: reqwest::Client,
}

impl ApiClient for HttpClient { /* ... */ }
```

---

### 4. Отсутствие обработки ошибок в коде

**Проблема:** Нужно проверить, как обрабатываются ошибки в `webapp.rs` и других файлах.

**Рекомендация:** Убедиться, что все функции возвращают `Result<T, E>`, а не `unwrap()`.

---

## 🔧 Рекомендации по рефакторингу

### Предлагаемая структура проекта

```
telegram-client/
├── Cargo.toml (workspace)
├── telegram-sdk/
│   ├── Cargo.toml
│   └── src/
│       ├── models/          # User, Chat, Message
│       ├── api/             # HTTP/WebSocket clients
│       ├── network/         # Connection management
│       ├── services/        # Business logic
│       ├── state/           # State management
│       ├── events/          # Event system
│       └── lib.rs
├── telegram-ui/
│   ├── Cargo.toml
│   └── src/
│       ├── components/      # UI components (Leptos/Yew)
│       ├── hooks/           # Custom hooks
│       ├── context/         # React-like context
│       └── lib.rs
├── examples/
│   ├── leptos/
│   └── yew/
└── app/
    ├── Cargo.toml
    └── src/
        ├── main.rs          # Entry point
        └── app.rs           # App initialization
```

### Пример контракта SDK ↔ UI

```rust
// telegram-sdk/src/lib.rs
pub struct TelegramSdk {
    api: Box<dyn ApiClient>,
    state: Arc<Mutex<AppState>>,
    events: EventManager,
}

impl TelegramSdk {
    pub fn new(api: Box<dyn ApiClient>) -> Self { /* ... */ }
    
    pub fn get_chats(&self) -> Result<Vec<Chat>,SdkError> { /* ... */ }
    pub fn get_messages(&self, chat_id: ChatId) -> Result<Vec<Message>, SdkError> { /* ... */ }
    pub fn send_message(&self, chat_id: ChatId, text: String) -> Result<(), SdkError> { /* ... */ }
    
    pub fn subscribe_events(&self) -> mpsc::Receiver<AppEvent> { /* ... */ }
}
```

### Пример UI-кода

```rust
// telegram-ui/src/components/chat_list.rs
#[component]
pub fn ChatList(sdk: UseRef<TelegramSdk>) -> impl IntoView {
    let chats = create_memo(move || sdk.read().get_chats().unwrap_or_default());
    
    view! {
        <List>
            {chats().iter().map(|chat| {
                view! {
                    <ChatListItem chat_id=chat.id />
                }
            }).collect::<Vec<_>>()}
        </List>
    }
}
```

---

## Дополнительная информация

Для более детального анализа мне нужны:

1. Код `webapp.rs` и `context/platform.rs`
2. Содержимое `Cargo.toml` в корне и в crates
3. Описание того, что планируется в `telegram-ui-core`
4. План по SDK (если есть)

---

*Отчёт сгенерирован на основе структуры проекта на 2024-2025*
