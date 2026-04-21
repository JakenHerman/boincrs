# App Controller

The `AppController` drives `boincrs` as a deterministic event loop:
- gathers input events
- updates in-memory state
- dispatches BOINC RPC reads/writes
- triggers terminal redraws

## High-level flow

```mermaid
flowchart TD
  inputTick[InputOrTick] --> router[ActionRouter]
  router -->|refresh| readApi[ReadApi]
  router -->|command| writeApi[WriteApi]
  readApi --> reducer[StateReducer]
  writeApi --> reducer
  reducer --> appState[AppState]
  appState --> renderer[Renderer]
```

## Confirmation flow for destructive actions

```mermaid
flowchart TD
  action[IncomingAction] --> destructive{DestructiveAction}
  destructive -->|No| execute[ExecuteImmediately]
  destructive -->|Yes| hold[StoreAsPending]
  hold --> confirm{UserConfirms}
  confirm -->|Yes| execute
  confirm -->|No| cancel[DiscardPendingAction]
```

## Reusability choices
- Read/write RPC logic is split into `BoincReadApi` and `BoincWriteApi`.
- Transport is abstracted by `BoincTransport`, so tests can inject a fake transport.
- UI rendering is isolated under `src/ui/**` and only consumes `AppState`.
