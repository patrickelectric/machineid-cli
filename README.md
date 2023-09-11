# machineid-cli

[![Test, Build and Deploy Images](https://github.com/patrickelectric/machineid-cli/actions/workflows/build_and_release.yml/badge.svg)](https://github.com/patrickelectric/machineid-cli/actions/workflows/build_and_release.yml)
![Downloads](https://img.shields.io/github/downloads/patrickelectric/machineid-cli/total?label=Downloads)

[![Latest Stable](https://img.shields.io/github/v/release/patrickelectric/machineid-cli.svg?label=Latest%20Stable)
![Date](https://img.shields.io/github/release-date/patrickelectric/machineid-cli?label=Date)](https://github.com/patrickelectric/machineid-cli/releases/latest)

Generate a 'device ID' that can be used to uniquely identify a computer based on files or hardware configuration.

```sh
Usage: machineid-cli [OPTIONS]

Options:
  -k, --key <KEY>
          Optional key to be used as encryption key

          [default: "Ski-bi da ba dibby du da ba dibby [?], do I'm the Scatman"]

  -t, --token-file <TOKEN_FILE>
          Sets a file that the content will be used on encryption

  -p, --parts <PARTS>...
          Parts to be used on ID generation, check 'everything' to use all parts

          [default: system-id]

          Possible values:
          - system-id:    System UUID
          - cpu-cores:    Number of CPU Cores
          - os-name:      Name of the OS
          - username:     Current Username
          - machine-name: Host machine name
          - mac-address:  Mac Address
          - cpuid:        CPU Vendor ID
          - drive-serial: UUID of the root disk

      --everything
          Use all parts

  -e, --encription <ENCRIPTION>
          Encryption to be used in key generation

          [default: md5]
          [possible values: md5, sha256, sha1]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```