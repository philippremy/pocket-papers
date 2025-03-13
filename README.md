# Pocket Papers
Ein Tool zum Erstellen von Hosentaschenkarten für Wettkämpfe im Rhönradturnen.

## Project Structure
```Go
pocket-papers // Project Root
│
└──.github // GitHub Actions
│
└──backend // Backend
│   │
│   └──.cargo // Cargo configuration
│   │
│   └──backend-server // Actix Server
│   │
│   └──backend-serverd // Daemon
│   │
│   └──external // Third-party tools
│       │
│       └──rust-rpxy // Reverse Proxy Tool
│       │
│       └──typst // Typst (PDF Compiler)
│
└──components // Shared frontend components
│   │
│   └──meta // HTML Head Component
│   │
│   └──navbar // NavBar component
│
└──islands // Islands (require JavaScript)
│   │
│   └──data // Form and Importer Components
│   │
│   └──primitives // Input Wrapper Components
│   │
│   └──table // Main Table Component
│
└──res // Development Resources
│
└──static // Statically served files
│
└──utils // Shared Preact-Signals file
```

## Frontend Runtime/Development Dependencies
- [x] Deno (^2.2.3)

## Frontend Run with HMR
`deno task start`

## Backend Development Dependencies
- [x] rustc (^1.86.0) [^1]
- [x] Cargo (^1.86.0) [^1]
- [x] Rust stdlib (^1.86.0) [^1]

- [x] mold linker (^2.37.0) [^2]

[^1]: Available with the rustup tool.

## Backend Runtime Dependencies
- [x] `Arial.ttf`
- [x] `Arial Bold.ttf`
- [x] `Arial Black.ttf`
- [x] `TwitterColorEmoji-SVGinOT.tff` [^3]

## Backend Release Build
`cd backend/backend-serverd && cargo build --release`

## Backend Release Run
Build first, see above!

`mkdir -p $HOME/.local/share` [^2]

`./backend-serverd`

[^2]: Linux only!

## Common Issues
- Permission Denied: OSError 13
    - rust-rpxy (`rpxy`) will bind to lower ports (i.e, 80 for HTTP and 443 for HTTPS), which is a priviledged operation on most UNIX-like systems. On Linux the tool can be whitelisted explicitly: `sudo setcap CAP_NET_BIND_SERVICE=+eip ./rpxy`. Depending on the system, the host daemon might need to be whitelisted as well: `sudo setcap CAP_NET_BIND_SERVICE=+eip ./backend-serverd`.

- No such file or directory: OSError 2
    - On Linux, the daemon and the backend-server expects that the user data dir exists. Ensure that you run `mkdir -p $HOME/.local/share` before launching the daemon or the backend-server.

- There is no STDOUT and STDERR in the tty
    - STDOUT and STDERR of the backend-server are rerouted to a text file by default (persisted logging). The file is located in `<DATA_DIR>/Logs`.

- The created pocket paper does not look right
    - `typst-cli` expects that `Arial.ttf`, `Arial Bold.ttf`, `Arial Black.ttf` and `TwitterColorEmoji-SVGinOT.tff` [^3] are installed on the server. Install these fonts, reload the font-cache and everything should be working.

[^3]: Available for download at: https://github.com/13rac1/twemoji-color-font.