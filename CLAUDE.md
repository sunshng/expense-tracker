# 记账APP - 产品文档

## 项目概述

- **项目名称**：记账APP
- **目标平台**：Windows 桌面 + Mac 桌面 + 手机 PWA（iOS/Android）
- **技术栈**：Tauri 2 + Vue 3 + Rust + SQLite/IndexedDB + Element Plus + ECharts
- **货币单位**：人民币（¥）
- **线上地址**：https://sunshng.github.io/expense-tracker/
- **代码仓库**：https://github.com/sunshng/expense-tracker

---

## 沟通规范（极其重要！）

> ⚠️ **用户是非技术背景的产品需求方。在整个项目开发过程中，任何技术相关的决策（技术选型、架构方案、工具选择、UI组件库选择、第三方库选择等），Claude 必须主动列出 2-3 个可行方案，用通俗易懂的语言向用户解释每个方案的优劣势和适用场景，最后由用户拍板决定。不得擅自替用户做技术决策。**

此规范适用于本项目的整个生命周期，任何违反此规范的决策均视为无效。

---

## 核心功能（全部完成 ✅）

### P0 - 必须有（第一期开发）
- [x] 记一笔支出（金额、日期、分类、备注）
- [x] 两级分类选择（一级大类 + 二级小类）
- [x] 账单列表展示（按日期倒序排列）
- [x] 数据本地存储（SQLite / IndexedDB）

### P1 - 很重要（第二期开发）
- [x] 月度收支统计
- [x] 分类占比图表（饼图）
- [x] 编辑/删除账单
- [x] 记录收入

### P2 - 可以有（第三期开发）
- [x] 数据导出为 CSV（账单列表页 + 设置页均可导出）
- [x] 年度统计（月度/季度/年度时间切换）
- [x] 月度预算设置（可设置上限、进度条超支警告）
- [x] 关键词搜索（按备注搜索账单）

### P3 - 锦上添花（远期规划）
- [x] 多账户管理（现金/储蓄卡/信用卡/支付宝/微信/投资账户）
- [x] 周期性账单提醒（房租/工资等定期提醒）
- [x] 数据备份恢复（JSON 导出/导入，可跨设备迁移）
- [x] 主题切换（浅色模式 / 深色模式）

---

## 两级分类体系

### 🍽️ 餐饮
早餐、午餐、晚餐、零食饮料、聚餐请客、外卖

### 🚗 交通
公交地铁、出租车/网约车、高铁/火车、飞机、加油、停车费

### 🛒 购物
日用品、数码产品、家居用品、宠物用品、办公用品

### 🏠 住房
房租、水电费、燃气费、物业费、维修、网费

### 🎮 娱乐
电影、游戏、旅游、运动健身、KTV/酒吧、书籍

### 🏥 医疗
门诊、药品、体检、住院、牙科

### 📚 教育
培训课程、书本教材、考试报名、文具

### 📱 通讯
话费、快递

### 👗 服饰
衣服、鞋子、包包、饰品、化妆品

### 👶 亲子
奶粉、尿布、玩具、教育

### 🎁 人情
送礼、红包、捐款、婚礼份子

### 🔧 其他支出
不确定分类

### 💰 收入（15 个来源）
工资薪金、年终奖金、绩效提成、兼职收入、自由职业、经营收入、投资收益、理财收益、租金收入、生活补贴、红包收入、礼金收入、报销退款、出售闲置、其他收入

---

## 数据模型

### 账单表 (transactions)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键，自增 |
| type | TEXT | 'expense' 或 'income' |
| amount | REAL | 金额（元） |
| category_l1 | TEXT | 一级分类 |
| category_l2 | TEXT | 二级分类 |
| date | TEXT | 日期 (YYYY-MM-DD) |
| note | TEXT | 备注（可选） |
| created_at | TEXT | 创建时间戳 |

### 分类表 (categories)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键，自增 |
| name | TEXT | 分类名称 |
| parent_id | INTEGER | 父分类ID（一级分类为 NULL） |
| icon | TEXT | 图标 Emoji |
| sort_order | INTEGER | 排序 |

---

## 技术架构

```
记账APP/
├── public/              # PWA 图标等静态资源
├── pwa-output/          # PWA 手机版构建输出（可直接部署）
├── src/                 # Vue 3 前端源码
│   ├── App.vue          # 根组件（桌面侧边栏 + 手机底部导航）
│   ├── main.js          # Vue 入口
│   ├── db/              # 统一数据库接口
│   │   ├── index.js     # 自动检测环境（Tauri 或 PWA）
│   │   ├── tauri.js     # Tauri 端（调用 Rust 后端）
│   │   └── browser.js   # PWA 端（IndexedDB 浏览器存储）
│   ├── router/index.js  # 路由（首页/记一笔/统计/设置）
│   ├── store/index.js   # Pinia 状态（主题/预算/账户/提醒）
│   ├── views/           # 4 个页面组件
│   │   ├── HomeView.vue     # 账单列表（筛选/搜索/编辑/删除/导出）
│   │   ├── AddRecord.vue    # 记一笔（支出/收入 + 两级分类）
│   │   ├── Statistics.vue   # 统计分析（饼图/趋势图/排行）
│   │   └── SettingsView.vue # 设置（预算/账户/提醒/备份/主题）
│   └── assets/styles.css   # 全局样式（浅色/深色主题）
├── src-tauri/           # Tauri 桌面端（Rust 后端）
│   ├── src/
│   │   ├── main.rs      # Rust 入口
│   │   ├── lib.rs       # 命令处理器（6 个 IPC 接口）
│   │   └── database.rs  # SQLite 数据库（建表/CRUD/统计）
│   └── tauri.conf.json  # Tauri 配置
├── target/              # 输出目录
│   ├── 记账APP_Setup.exe # Windows 安装包（3.6 MB）
│   └── 作用.md           # 各文件夹作用说明
├── CLAUDE.md            # 本文档
├── package.json         # npm 配置
├── vite.config.js       # 构建配置（含 PWA 插件）
└── index.html           # HTML 入口
```

### 数据流向

```
用户界面 (Vue 3)
    ↓
src/db/index.js — 自动检测运行环境
    ↓
┌─────────────────┬─────────────────┐
│  PWA（浏览器）    │  Tauri（桌面端）  │
│  IndexedDB 存储  │  invoke() 调用   │
│  (db/browser.js) │  Rust 后端 SQLite │
└─────────────────┴─────────────────┘
```

---

## 使用命令

```bash
# 开发
npm run dev          # 启动前端开发服务器（浏览器可预览 PWA）
npm run tauri dev    # 启动 Tauri 桌面版开发

# 构建
npm run build        # 构建 PWA 到 dist/
npm run tauri build  # 构建桌面版安装包

# 预览
npm run preview      # 本地预览 PWA
```

---

## UI 设计原则

- 简洁直观，适合非技术用户
- 所有文案使用中文
- 金额显示精确到分（0.00）
- 支持键盘快捷操作（回车提交等）
- 响应式布局：桌面侧边栏 / 手机底部导航
- 支持浅色/深色主题切换

---

## 决策日志

| 日期 | 决策项 | 选择 | 原因 |
|------|--------|------|------|
| 2026-07-17 | 技术栈方案 | Electron + Vue 3 | 用户选择：生态成熟、社区庞大、UI组件库丰富 |
| 2026-07-17 | 技术栈变更 | Tauri 2 + Vue 3 | Electron Win 已知Bug（#49034），无法正常使用 |
| 2026-07-17 | 数据库方案 | rusqlite + IndexedDB | 桌面 SQLite 内嵌编译；PWA 用浏览器 IndexedDB |
| 2026-07-17 | 打包工具 | NSIS | WiX 需 .NET 组件；NSIS 更稳定 |
| 2026-07-17 | 手机版方案 | PWA | 代码改动最小，iPhone/Android 通用，离线可用 |
| 2026-07-17 | 部署平台 | GitHub Pages | Gitee Pages 界面异常；GitHub Pages 稳定 |
| 2026-07-17 | 分类优化 | 收入扩展到15个 | 用户反馈收入来源太少，从7个增至15个 |
