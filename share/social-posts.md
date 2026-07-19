# 🐘 Share Post Drafts

## 🐦 Twitter / X

**Post 1 (Launch):**
> 🚀 Just published a FREE comprehensive book on Rust game physics math with Bevy!
>
> 🧮 Vectors → Matrices → Quaternions → Trig
> 🏃 Kinematics → Dynamics → Integration
> 💥 Collision Detection → Response → Constraints
> 🏗️ Full ECS physics architecture
> 🎮 Interactive physics sandbox demo
>
> 16 chapters, 7K+ lines of idiomatic Rust code.
> All from scratch — no physics crate needed.
>
> 📖 https://github.com/arpanpathak/bevy-physics-book
>
> #rustlang #bevy #gamedev #physics

**Post 2 (Deep dive):**
> The single most underrated physics concept in game dev: the Semi-Implicit (Symplectic) Euler integrator.
>
> v += a·dt
> x += v·dt   ← uses NEW v!
>
> This tiny change vs Explicit Euler means energy is conserved. No exploding physics! 💥
>
> I cover this (and much more) in my free Rust/Bevy physics book:
> https://github.com/arpanpathak/bevy-physics-book
>
> #rustlang #gamedev

## 🧵 Reddit (r/rust)

**Title:** I wrote a comprehensive book on game physics math with Bevy & Rust

**Body:**
Hey r/rust! 👋

I just finished writing a **comprehensive book** on building physics engines from scratch using Rust and the Bevy game engine. It's completely free and open-source.

**What's covered (16 chapters, ~7K lines of markdown + code):**

- **📐 Math Foundation:** Vectors, matrices, quaternions, trig
- **🏃 Motion Physics:** Kinematics, dynamics, forces (F=ma)
- **🔄 Integration:** Euler, Verlet, RK4, sub-stepping
- **💥 Collisions:** Detection (AABB, Circle, SAT) + Response (impulses)
- **🔗 Advanced:** Constraints, ragdolls, spatial partitioning
- **🏗️ Architecture:** Full Bevy ECS plugin design
- **🎮 Demo:** Complete interactive physics sandbox

**Key features:**
- ✅ All code is copy-paste runnable with Bevy 0.15
- ✅ Extensive ASCII diagrams for visual learning
- ✅ Generous comments explaining WHY (not just what)
- ✅ Everything from scratch — no physics crate dependency
- ✅ Compilable crate workspace + standalone sandbox
- ✅ mdBook format with GitHub Pages deployment

**Repo:** https://github.com/arpanpathak/bevy-physics-book

Would love feedback, PRs, and issues! 🦀🎮

## 🎮 HackerNews

**Title:** Show HN: I wrote a free book on game physics math with Rust and Bevy

**Body:**
I've been building game physics in Rust for a while and realized most resources either assume too much math background or skip the math entirely. So I wrote a book that starts from vectors and builds up to a complete physics sandbox.

The repo includes:
- 16 markdown chapters
- compilable Rust workspace (13 crates, one per chapter)
- standalone physics sandbox game
- mdBook config for GitHub Pages

All code is idiomatic Rust with Bevy 0.15, heavily commented, and designed to be understood by someone who knows basic Rust but not necessarily game physics.

https://github.com/arpanpathak/bevy-physics-book

Happy to answer questions!
