# Anki Sync Server

这个目录提供一个自托管 Anki 同步服务。它不依赖第三方 Anki Docker 镜像，而是从官方
PyPI `anki` 包构建一个最小镜像，并把同步数据持久化到 `./data/`。

## 使用

```bash
cd docker/anki-server
cp .env.example .env
```

编辑 `.env`，至少把 `SYNC_USER1` 改成自己的账号密码：

```dotenv
SYNC_USER1=your-name:your-strong-password
```

启动服务：

```bash
docker compose up -d --build
docker compose logs -f
```

默认监听本机 `8080` 端口，数据保存在 `docker/anki-server/data/`。

## Anki 客户端设置

在 Anki 桌面端的 Preferences -> Syncing 中，把 self-hosted sync server 设置为：

```text
http://<服务器IP>:8080/
```

账号密码使用 `.env` 里的 `SYNC_USER1`。如果通过反向代理提供 HTTPS，请在 Anki 里填写
代理后的 HTTPS 地址，并保留末尾 `/`。

## 配置项

| 变量 | 说明 |
| --- | --- |
| `ANKI_VERSION` | 构建时安装的 `anki` PyPI 包版本。客户端升级后，建议同步升级服务端版本。 |
| `ANKI_SYNC_PORT` | 宿主机暴露端口，默认 `8080`。 |
| `SYNC_USER1` | 第一个同步账号，格式为 `username:password`。 |
| `MAX_SYNC_PAYLOAD_MEGS` | 单次上传大小限制，默认 `100`。 |
| `TZ` | 容器时区，默认 `Asia/Shanghai`。 |

## 多用户

Anki 服务支持 `SYNC_USER2`、`SYNC_USER3` 等环境变量。如果需要多用户，继续在 `.env`
里添加：

```dotenv
SYNC_USER2=another-user:another-password
```

## 注意

- 不要把服务器的 `data/` 目录指向本机正常使用的 Anki 数据目录。
- 这个服务默认是未加密 HTTP，不建议直接暴露到公网；更适合局域网、VPN/Tailscale，
  或放在 HTTPS 反向代理后面。
- `.env` 和 `data/` 已被本目录 `.gitignore` 忽略，避免提交密码和同步数据。
