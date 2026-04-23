![boincrs banner](boincrs.jpg)

# boincrs

A fast, keyboard-first **Rust terminal UI** for a **local BOINC client** over
the GUI RPC interface. **Beta** — PrimeGrid and Asteroids@home auto-attach via
account keys are supported.

**Full documentation lives at [jakenherman.github.io/boincrs](https://jakenherman.github.io/boincrs).**

- [Why boincrs?](https://jakenherman.github.io/boincrs/guide/why-boincrs)
- [Getting started](https://jakenherman.github.io/boincrs/guide/getting-started)
- [Configuration](https://jakenherman.github.io/boincrs/guide/configuration)
- [Project templates & profiles](https://jakenherman.github.io/boincrs/guide/architecture/project-templates-and-profiles)
- [Keyboard reference](https://jakenherman.github.io/boincrs/guide/keyboard)
- [Compatibility matrix](https://jakenherman.github.io/boincrs/guide/compatibility)

## Quick install (from source)

```bash
git clone https://github.com/jakenherman/boincrs.git
cd boincrs
cargo build --release
```

Binary: `./target/release/boincrs` (Linux/macOS) or
`.\target\release\boincrs.exe` (Windows).

## Quick run

```bash
cp .env.example .env          # fill in BOINCRS_PASSWORD_FILE + optional keys
cargo run
```

`boincrs` auto-loads `.env` at startup if one exists in the working directory.
See the [Configuration guide](https://jakenherman.github.io/boincrs/guide/configuration)
for every environment variable.

## Requirements

- [BOINC](https://boinc.berkeley.edu/download.php) client running locally with
  GUI RPC enabled.
- [Rust](https://rustup.rs/) (stable) and `cargo`.
- GUI RPC password (often `/etc/boinc-client/gui_rpc_auth.cfg` on Linux), or
  `BOINCRS_PASSWORD`.
- Optional: project **authenticator** keys for auto-attach.

## Testing

```bash
cargo test
```

Live-daemon and integration test variants are documented on the
[Testing page](https://jakenherman.github.io/boincrs/guide/testing).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and the
[Contributing guide](https://jakenherman.github.io/boincrs/guide/contributing).

## Changelog

[`CHANGELOG.md`](CHANGELOG.md) (also mirrored on the
[docs site](https://jakenherman.github.io/boincrs/guide/changelog)).

## Support & security

- Sponsor: [`SUPPORT.md`](SUPPORT.md)
- Do **not** commit real `.env` values, GUI RPC passwords, or project
  authenticators. Treat them as secrets.

## License

MIT — see [`LICENSE`](LICENSE).
