// Tauri 端数据库 - 通过 invoke 调用 Rust 后端
import { invoke } from '@tauri-apps/api/core'

export async function getCategories() {
  return await invoke('get_categories')
}

export async function getTransactions(filters = {}) {
  return await invoke('get_transactions', { filters })
}

export async function addTransaction(transaction) {
  return await invoke('add_transaction', { transaction })
}

export async function deleteTransaction(id) {
  return await invoke('delete_transaction', { id })
}

export async function updateTransaction(id, transaction) {
  return await invoke('update_transaction', { id, transaction })
}

export async function getStatistics(startDate, endDate) {
  return await invoke('get_statistics', { startDate, endDate })
}
