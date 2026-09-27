# Lapapp

[English](./README.md) | [简体中文](./README.zh.md)

基于 **Tauri 2 + Vue 3 + TypeScript + [lapstyle](../../Web/lapstyle)** 的桌面应用基础框架，适合作为新项目的起点。

- 前端样式与组件来自 lapstyle（`Ls*` 组件库 + CSS 设计令牌）
- 借鉴 Lapeditor 的无边框窗口与窗口状态记忆方案

## 功能

- **无边框窗口**：Windows 下移除系统装饰、自绘标题栏，整栏可拖拽，双击最大化；其余平台保留系统边框
- **窗口状态记忆**：位置、尺寸、最大化状态自动保存（移动/缩放停止 1s 后 + 关闭时兜底），下次启动先恢复再显示，无跳动
- **界面缩放**：Ctrl + 加/减/0 与 Ctrl + 滚轮（0.5–2.0），记忆在 `settings.json`
- **多主题**：lapstyle 全部 7 套主题（dark / light / mint / sky / pink / brown / amber），右上角圆形按钮一键切换
- **多语言**：内置简体中文 / English，右上角圆形按钮切换，即时生效
- **UI 示例页**：组件示例（lapstyle 控件演示）、设置（主题 / 语言 / 关于）

主题与语言持久化到 `config/settings.json`；Rust 会在页面加载前注入主题属性，避免启动闪烁。

## 环境要求

- Node.js 20+ 与 [pnpm](https://pnpm.io)
- Rust（stable，Windows 需 MSVC 工具链）
- lapstyle 仓库需位于 `../../Web/lapstyle`（相对本项目），见 `package.json` 中的 `file:` 依赖

## 快速开始

```bash
pnpm install
pnpm dev        # 开发模式（前端 vite 端口 1430，避免与 Lapeditor 的 1420 冲突）
pnpm build      # 打包安装程序
```

其他命令：

| 命令 | 说明 |
| --- | --- |
| `pnpm dev:web` | 仅启动前端（浏览器调试，无 Tauri 外壳） |
| `pnpm build:web` | 类型检查 + 前端产物构建 |
| `pnpm typecheck` | vue-tsc 类型检查 |
| `pnpm tauri icon <svg>` | 重新生成各平台应用图标 |

## 项目结构

```
├── index.html                  # 入口 HTML（内联主题预加载脚本，防闪烁）
├── src/
│   ├── main.ts                 # 应用入口：lapstyle CSS/插件 + i18n + 窗口状态绑定
│   ├── App.vue                 # 布局骨架：标题栏 + 侧边栏 + 内容区
│   ├── styles.css              # 应用样式（覆盖在 lapstyle 令牌之上）
│   ├── settings.ts             # 主题/语言状态与持久化（调用 Rust 命令）
│   ├── i18n.ts                 # vue-i18n 配置与语言清单
│   ├── windowState.ts          # 移动/缩放防抖保存窗口几何
│   ├── zoom.ts                 # Ctrl+±/0 与 Ctrl+滚轮缩放
│   ├── components/
│   │   ├── TitleBar.vue        # 自绘标题栏：语言/主题圆形按钮 + 窗口控制按钮
│   │   └── SideNav.vue         # 左侧菜单
│   ├── views/                  # 页面（左侧菜单选中后渲染在右侧内容区）
│   │   ├── WidgetsView.vue
│   │   └── SettingsView.vue
│   └── locales/                # zh.ts / en.ts 文案
└── src-tauri/
    ├── tauri.conf.json         # 窗口默认尺寸、devUrl(1430)、打包配置
    ├── capabilities/default.json
    └── src/
        ├── lib.rs              # Tauri setup：无边框、恢复几何、命令注册
        ├── config.rs           # settings.json / window.json 读写与恢复逻辑
        └── paths.rs            # 可移植目录布局（exe / 仓库根目录下的 config/）
```

## 新增页面

1. 在 `src/views/` 新建组件，如 `AboutView.vue`
2. 在 `src/App.vue` 的 `pages` 中注册：`about: AboutView`
3. 在 `src/components/SideNav.vue` 加一个 `<ls-menu-item value="about">`
4. 在 `src/locales/zh.ts`、`en.ts` 补充文案

## 运行时配置

可移植设计，均位于运行目录下（开发期为仓库根目录）的 `config/`：

| 文件 | 内容 |
| --- | --- |
| `window.json` | `x / y / width / height / maximized` |
| `settings.json` | `theme / locale` |

## 实现要点（自 Lapeditor 移植）

- Tauri 的 `set_size()` 是内尺寸、`set_position()` 是外位置；最大化时用 Win32 `GetWindowPlacement` 读取恢复区，并按装饰差值换算，避免窗口每次启动被 DWM 边框撑大
- 最小化时 Windows 会报告 `-32000` 一类坐标，保存与恢复均有合法性校验
- 窗口初始 `visible: false`，恢复几何后再 `show()`，避免可见的位置跳动
- 仅在 Windows 使用自绘标题栏（`uses_custom_titlebar` 命令），其余平台走系统装饰
