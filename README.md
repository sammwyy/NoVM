# 🚫🖥️ NoVM

NoVM is a tool designed to hide virtual machines (VMs) from malicious actors.

## ✨ Features

- **Cloak Virtual Machines:** Protect your VM by obfuscating registry entries, files, drivers, and processes.
- **Cross-Platform Support:** Automatically detects the VM platform or allows manual selection.

## 📜 Usage

To use NoVM, you can run the following command:

> [!WARNING]
> Do not run this software in a real environment, as it may destabilize your system or render it unusable. By default, NoVM will not run if it does not detect any traces of a virtualizing agent.

```bash
novm --help
```

> [!IMPORTANT]
> On Linux you must run the executable as sudo, on Windows as NT authority (You can use [PsExec](https://learn.microsoft.com/es-es/sysinternals/downloads/psexec) for this)

### Command Line Options

- `-p, --platform <auto|vmware|virtualbox>`: Specifies the platform to cloak. Default is `auto`.
- `--no-reg`: Do not cloak registry entries.
- `--no-files`: Do not obfuscate files.
- `--no-kill`: Do not kill processes.
- `--no-drivers`: Do not obfuscate drivers.

> [!TIP]
> The "auto" option allows the virtualizer to be detected using the running processes. If you want to run NoVM more than once you must force the cleanup function by specifying the platform using the "-p" parameter.

## Compatibility Table 🛠️

| Guest OS   | VMware | VirtualBox | QEMU | Hyper-V | KVM | Xen |
|------------|:------:|:----------:|:----:|:-------:|:---:|:---:|
| Windows    | ✔️     | ❌        | ❌  | ❌     | ❌ | ❌  |
| Linux      | ❌     | ❌        | ❌  | ❌     | ❌ | ❌  |
| macOS      | ⚠️     | ⚠️        | ⚠️  | ⚠️     | ⚠️ | ⚠️  |

> ✔️: Supported
> ❌: Not Supported (yet)
> ⚠️: Not planned

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check [issues page](https://github.com/sammwyy/novm/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you! Or buy me a coffeelatte 🙌 on [Ko-fi](https://ko-fi.com/sammwy)

## 📝 License

Copyright © 2024 [Sammwy](https://github.com/sammwyy). This project is [MIT](LICENSE) licensed.
