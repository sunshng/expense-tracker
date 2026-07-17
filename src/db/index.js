// 统一数据库接口 - 自动检测运行环境（Tauri 桌面端 或 PWA 浏览器端）

let db

// 检测是否在 Tauri 环境中
function isTauri() {
  return !!(window.__TAURI_INTERNALS__ || window.__TAURI__)
}

export async function initDB() {
  if (db) return db

  if (isTauri()) {
    const tauri = await import('./tauri.js')
    db = tauri
  } else {
    const browser = await import('./browser.js')
    db = browser
  }
  return db
}

// 代理所有数据库调用
async function getDB() {
  if (!db) {
    db = await initDB()
  }
  return db
}

export async function getCategories() {
  const d = await getDB()
  return d.getCategories()
}

export async function getTransactions(filters) {
  const d = await getDB()
  return d.getTransactions(filters)
}

export async function addTransaction(transaction) {
  const d = await getDB()
  return d.addTransaction(transaction)
}

export async function deleteTransaction(id) {
  const d = await getDB()
  return d.deleteTransaction(id)
}

export async function updateTransaction(id, transaction) {
  const d = await getDB()
  return d.updateTransaction(id, transaction)
}

export async function getStatistics(startDate, endDate) {
  const d = await getDB()
  return d.getStatistics(startDate, endDate)
}
