# Devflo

## What is Devflo and Who Is It For

Devflo is a simple tool that checks your project for secrets and risky files before you ship. It’s for developers who want to know if their app is safe without diving into technical details.

## Installation
For now, you can install DevFlo with Cargo (Rust’s package manager):
`cargo install devflo`
(We’ll update this when DevFlo is officially published.)

## Usage
Scan your project by running:
`devflo scan .`
Example output:
Found 2 issues:

1. File: ./src/config.yml <br/>
   Line: 12 <br/>
   Type: API Key <br/>
   Matched Text: "AKIAXXXXX" <br/>
   Severity: High <br/>

2. File: ./workflows/deploy.yml <br/>
   Line: 8 <br/>
   Type: Password <br/>
   Matched Text: "supersecret" <br/>
   Severity: Critical <br/>
This shows the file, line number, type of secret, the matched text, and how serious the issue is.
