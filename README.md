# Judy-2D - The BodyGen.re Bot

## Introduction

Judy-2D is the second iteration of [JewD](https://github.com/bodygenre/JewD). It's an all-in-one bot that does everything from searching for movies to generating Markov chain sentences. It's tightly coupled to BodyGen.re infrastructure at the moment and isn't really meant as a general purpose bot on other servers. Architecturally, it's built around a plugin system so that BodyGen.re users can easily create a plugin with new features, submit a PR, and have it work in the server. The main plugin launcher deals with Discord API authentication, environment variables, etc. Currently it's only written in Rust, but the intention is to use https://pyo3.rs/latest/python_from_rust.html or similar to run Python modules as well.

## How to Build and Run

0. Join [#coding-bardos](https://discord.com/channels/788925423420964904/979894864709824582) on [BodyGen.re's Discord](https://discord.gg/bfd4nWAAC6). Hang out and tell us what cool ideas you have for Judy or what sort of coding things you want to learn about

1. Install [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

2. Clone this repo and change into it
  ```bash
  git clone https://github.com/bodygenre/judy2d.git && cd judy2d
  ```

3. Get the environment deets from one of the admins and put it in a file called `.env`. It consists of key-value pairs of the form:
  ```bash
  APP_ID=<app_id>
  GUILD_ID=<guild_id>
  DISCORD_TOKEN=<discord_token>
  PUBLIC_KEY=<public_key>
  ```

4. Load the environment variables and run it
  ```bash
  export $(cat .env | xargs) && cargo run
  ```

## Join our Server!
[https://discord.gg/bfd4nWAAC6](https://discord.gg/bfd4nWAAC6)