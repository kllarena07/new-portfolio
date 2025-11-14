# Kieran Llarena's Portfolio

## About

This is the code repository for my portfolio website, specifically the terminal version of it. You can visit it at `ssh krayon.dev`.

To view the website version, use HTTPS instead of SSH..or just see https://krayon.dev. The code is publicly available [here](https://github.com/kllarena07/portfolio-v2-website)

## Portfolio Structure

```
portfolio-v2/
├── hikari-dance/                    # Animation frames for the terminal interface
│   ├── frame_0.png through frame_67.png
│   └── frames_cache.bin
├── src/
│   ├── pages/                       # Terminal page components
│   │   ├── labels/                  # Technology/service labels
│   │   │   ├── aws/                 # AWS service labels
│   │   │   │   ├── eventbridge_scheduler.rs
│   │   │   │   ├── lambda.rs
│   │   │   │   ├── s3.rs
│   │   │   │   ├── sagemaker.rs
│   │   │   │   └── mod.rs
│   │   │   ├── cloudflare/          # Cloudflare service labels
│   │   │   │   ├── images.rs
│   │   │   │   ├── workers.rs
│   │   │   │   └── mod.rs
│   │   │   ├── react/               # React ecosystem labels
│   │   │   │   ├── react.rs
│   │   │   │   ├── react_native.rs
│   │   │   │   └── mod.rs
│   │   │   ├── container.rs         # Container technology labels
│   │   │   ├── expo.rs              # Expo framework labels
│   │   │   ├── fastapi.rs           # FastAPI labels
│   │   │   ├── flask.rs             # Flask labels
│   │   │   ├── groq.rs              # Groq AI labels
│   │   │   ├── javascript.rs        # JavaScript labels
│   │   │   ├── kinde_auth.rs        # Kinde auth labels
│   │   │   ├── label.rs             # Base label component
│   │   │   ├── modal.rs             # Modal component
│   │   │   ├── nextjs.rs            # Next.js labels
│   │   │   ├── pinecone.rs          # Pinecone labels
│   │   │   ├── pocketbase.rs        # PocketBase labels
│   │   │   ├── python.rs            # Python labels
│   │   │   ├── retell_ai.rs         # Retell AI labels
│   │   │   ├── supabase.rs          # Supabase labels
│   │   │   ├── sveltekit.rs         # SvelteKit labels
│   │   │   ├── tailwind.rs          # Tailwind CSS labels
│   │   │   ├── typescript.rs        # TypeScript labels
│   │   │   ├── vexo_analytics.rs    # Vexo Analytics labels
│   │   │   ├── websocket.rs         # WebSocket labels
│   │   │   └── mod.rs
│   │   ├── about.rs                 # About page
│   │   ├── experience.rs            # Experience page
│   │   ├── leadership.rs            # Leadership page
│   │   ├── mod.rs                   # Pages module
│   │   ├── page.rs                  # Base page component
│   │   ├── projects.rs              # Projects page
│   │   └── style.rs                 # Styling utilities
│   ├── server/                      # Server-side components
│   │   ├── app_server.rs            # Main application server
│   │   ├── mod.rs                   # Server module
│   │   └── terminal_handle.rs       # Terminal handling logic
│   ├── app.rs                       # Main application logic
│   └── main.rs                      # Application entry point
├── .gitignore                       # Git ignore file
├── Cargo.lock                       # Cargo lock file
├── Cargo.toml                       # Rust project configuration
├── give_perms.sh                    # Permission setup script
├── README.md                        # This file
└── start.sh                         # Application startup script
```

Made with ❤️ by [krayondev](https://x.com/krayondev)
