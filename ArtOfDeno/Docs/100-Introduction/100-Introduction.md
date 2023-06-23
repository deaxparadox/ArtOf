# Introduction

Deno is a JavaScript, TypeScript, and Web Assembly runtime with  secure defaults and a great deverlope experience.

It's build on V8, Rust, and Tokio.

## Feature highlights

- Provides `web platform funcionality` and adopts web platform standards. For example using ES modules, web workers, and support `fetch()`.
- Secure by default. No file, network, or enviornment access unless explicitly enabled.
- Supports `TypeScript` out of the box.
- Ships a single executable (`deno`).
- Provides build-in `development tooling` like a code formatter (`deno fmt`), a linter (`deno lint`), a test runner (`deno test`), and a `language server for your editor`.
- Has `a set of reviewed (audited) standard modules` that are generated to work with Deno.
- Supports the user of `existing npm modules`.

## Philosophy

Deno aims to be a productive and secure scripting environment for the modern programmer.

Deno will always be distributed as a single executable. Given a URL to a Deno program, it is runnable with nothing more than `the ~31 megabyte zipped executable`. Deno explicitly takes on the role of both runtime and package manager. It uses a standard browser-compatible protocol for loading modules: URLs.

## Goals

- Ship as just a single executable (deno).
- Provide secure defaults.
    - Unless specifically allowed, scripts can't access files,the environment, or the network.
- Be browser-compatible.
    - The subset of Deno programs which are written completely in JavaScript and do not use the global Deno namespace (or feature test for it), ought to also be able to be run in a modern web browser without change.
- Provide built-in tooling to improve developer experience.
    - E.g. unit testing, code formatting, and linting.
- Keep V8 concepts out of user land.
- Serve HTTP efficiently.

## Other key behaviors

- Fetch and cache remote code upon first execution, and never update it until the code is run with the --reload flag. (So, this will still work on an airplane.)
- Modules/files loaded from remote URLs are intended to be immutable and cacheable.
