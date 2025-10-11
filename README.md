# 🏋️ **IronPulse — Fitness Landing Page**

This is a modern and fully responsive fitness landing page built with **[Leptos](https://github.com/leptos-rs/leptos)** (Rust + WebAssembly).  
It features smooth UI components, progress tracking visuals, testimonials, and a clear call-to-action.

## ✨ Features

- 🚀 Fully responsive layout with **[Tailwind CSS](https://tailwindcss.com/)**
- 🧭 Sticky navigation bar with mobile menu
- 🦾 Hero section with call-to-action buttons
- 🏃 Featured workout cards with hover effects
- 📊 Animated progress tracking ring
- 🧍‍♂️ Testimonials section
- 📝 Simple footer with social links
- ⚡ Built using Rust + Leptos + WASM

---

## 🧰 Prerequisites

Before running the project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (latest stable)
- [Trunk](https://trunkrs.dev/)
- [wasm32-unknown-unknown](https://doc.rust-lang.org/cargo/reference/config.html) target

You can install them with:

```bash
# Install Trunk
cargo install trunk

# Add WebAssembly target
rustup target add wasm32-unknown-unknown
```

---

## 🛠️ Run Locally

1. **Clone the repo**

```bash
git clone https://github.com/your-username/ironpulse-leptos.git
cd ironpulse-leptos
```

2. **Run the development server**

```bash
trunk serve
```

By default, this will start the server at:

👉 [http://localhost:8080](http://localhost:8080)

Any code changes will hot-reload automatically.

---

## 🏗️ Build for Production

To create an optimized production build:

```bash
trunk build --release
```

This will generate the build in the `dist` folder.

---

## 📁 Project Structure

```
ironpulse-leptos/
├── src/
│   ├── main.rs             # Entry point of the app
│   └── components          # Your reusable components (Navigation, Hero, etc.)
├── index.html              # Entry HTML file
├── Cargo.toml              # Rust dependencies
└── Trunk.toml              # Trunk configuration file
```

---

## 🧪 Tech Stack

- 🦀 **Rust** — systems programming language
- 🌿 **Leptos** — full-stack Rust framework for the web
- 🌀 **Tailwind CSS** — utility-first CSS framework
- 🧰 **Trunk** — bundler and asset pipeline for Rust WASM apps

---

## 🧾 License

This project is licensed under the **MIT License**.
