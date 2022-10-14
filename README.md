# pro22kadai-image
## 実行方法
### Rustの環境構築をする
Windows 10/11の場合
```PowerShell
winget install rustup
```

※`winget`が見つからないと言われた場合は，Microsoft Storeから**アプリインストーラ**をインストールする

Arch Linuxの場合
```Bash
$ sudo pacman -S rustup
$ rustup default stable
```

その他のUnix/Linux系OSの場合
```Bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ビルドする
```Bash
$ git clone https://github.com/tnct-spc/pro22kadai-image -b main
$ cd pro22kadai-image/
$ make
```

その後に，同じ階層にできた`lambda.zip`をAWSに上げてください．
