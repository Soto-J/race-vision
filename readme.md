### This will start desktop UI
```bash
cd /desktop-app
npm run tauri dev
```

### This will start browser UI
```bash
cd /desktop-app
npm run dev
```

sqlx database create --database-url sqlite:db/app.db
sqlx migrate run
sqlx migrate add -r <name>: Use the -r flag to create a reversible migration