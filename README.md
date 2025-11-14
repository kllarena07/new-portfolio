# Kieran Llarena's Portfolio

## ✍ About

Inspired by [terminal.shop](https://www.terminal.shop/) and the brilliant minds at Terminal Products, Inc.

This is the code repository for my portfolio, specifically the terminal version of it.

You can visit it at `ssh krayon.dev` (use in a modern terminal for the best experience).

To view the website version, use HTTPS instead of SSH...or see https://krayon.dev. View the code [here](https://github.com/kllarena07/portfolio-v2-website)

## 📂 General Portfolio Structure

```
portfolio-v2/
├── hikari-dance/                    # Animation frames
│   ├── frame_0.png -> frame_67.png
│   └── frames_cache.bin             # Cached binary frame data
├── src/
│   ├── pages/                       # Portfolio pages
│   │   ├── labels/                  # Tech labels
│   │   ├── about.rs
│   │   ├── experience.rs
│   │   ├── leadership.rs
│   │   ├── mod.rs
│   │   ├── page.rs                  # Base page component
│   │   ├── projects.rs
│   │   └── style.rs                 # Styling utilities
│   ├── server/                      # Server-side components
│   │   ├── app_server.rs            # Main SSH server logic
│   │   ├── mod.rs                   
│   │   └── terminal_handle.rs       # Terminal handling logic
│   ├── app.rs                       # Main app logic
│   └── main.rs
└── other utilities
```

## 👾 Bugs or vulnerabilities

If you find any bugs or vulnerabilities, please contact me on my Twitter using the link below.

_Made with ❤️ by [krayondev](https://x.com/krayondev)_
