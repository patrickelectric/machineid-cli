# machineid-cli

[![Test, Build and Deploy Images](https://github.com/patrickelectric/machineid-cli/actions/workflows/build_and_release.yml/badge.svg)](https://github.com/patrickelectric/machineid-cli/actions/workflows/build_and_release.yml)
![Downloads](https://img.shields.io/github/downloads/patrickelectric/machineid-cli/total?label=Downloads)

[![Latest Stable](https://img.shields.io/github/v/release/patrickelectric/machineid-cli.svg?label=Latest%20Stable)
![Date](https://img.shields.io/github/release-date/patrickelectric/machineid-cli?label=Date)](https://github.com/patrickelectric/machineid-cli/releases/latest)

Generate a 'device ID' that can be used to uniquely identify a computer based on files or hardware configuration.

Possible sources (can use multiple sources):
- system-id: System UUID
- cpu-cores: Number of CPU Cores
- os-name: Name of the OS
- username: Current Username
- machine-name: Host machine name
- mac-address: Mac Address
- cpuid: CPU Vendor ID
- drive-serial: UUID of the root disk
- file: Content