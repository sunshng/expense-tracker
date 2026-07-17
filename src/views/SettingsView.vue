<template>
  <div class="settings">
    <h2 class="page-title">设置</h2>

    <!-- 月度预算 -->
    <el-card class="card-gap">
      <template #header><span>💰 月度预算</span></template>
      <div class="budget-row">
        <span>每月预算上限：</span>
        <el-input-number v-model="budget" :min="0" :step="100" :precision="0" placeholder="0表示不限制" />
        <span>元</span>
        <el-button type="primary" @click="saveBudget">保存</el-button>
      </div>
      <div class="budget-status" v-if="budget > 0 && monthExpense > 0">
        <el-progress :percentage="budgetPercent" :color="budgetPercent > 100 ? '#f56c6c' : '#67c23a'" />
        <span class="budget-text">本月已支出 ¥{{ monthExpense.toFixed(2) }} / ¥{{ budget }}</span>
        <span v-if="budgetPercent > 100" class="budget-warn">⚠️ 已超支！</span>
      </div>
    </el-card>

    <!-- 多账户管理 -->
    <el-card class="card-gap">
      <template #header><span>💳 账户管理</span></template>
      <el-table :data="store.accounts" style="width:100%" empty-text="暂无账户">
        <el-table-column prop="name" label="账户名称" />
        <el-table-column prop="type" label="类型" width="120">
          <template #default="scope">
            <el-tag size="small">{{ scope.row.type }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="80">
          <template #default="scope">
            <el-button size="small" type="danger" text @click="store.deleteAccount(scope.row.id)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
      <div class="add-row">
        <el-input v-model="newAccount.name" placeholder="账户名（如：工商银行）" style="width:180px" />
        <el-select v-model="newAccount.type" placeholder="类型" style="width:130px">
          <el-option label="💵 现金" value="现金" />
          <el-option label="🏦 储蓄卡" value="储蓄卡" />
          <el-option label="💳 信用卡" value="信用卡" />
          <el-option label="📱 支付宝" value="支付宝" />
          <el-option label="📱 微信" value="微信" />
          <el-option label="📊 投资账户" value="投资账户" />
        </el-select>
        <el-button type="primary" @click="addAccount">添加</el-button>
      </div>
    </el-card>

    <!-- 周期提醒 -->
    <el-card class="card-gap">
      <template #header><span>🔔 周期提醒</span></template>
      <div class="reminder-list" v-if="store.reminders.length > 0">
        <div v-for="r in store.reminders" :key="r.id" class="reminder-item">
          <span>{{ r.icon }} {{ r.name }}</span>
          <span class="reminder-info">{{ r.cycle }} · ¥{{ r.amount }}</span>
          <el-button size="small" type="danger" text @click="store.deleteReminder(r.id)">删除</el-button>
        </div>
      </div>
      <div v-else class="empty-hint">暂无提醒，添加房租、工资等周期性账单</div>
      <div class="add-row">
        <el-input v-model="newReminder.name" placeholder="名称（如：房租）" style="width:160px" />
        <el-input-number v-model="newReminder.amount" :min="0" :precision="2" placeholder="金额" style="width:130px" />
        <el-select v-model="newReminder.cycle" placeholder="周期" style="width:120px">
          <el-option label="每月" value="每月" />
          <el-option label="每周" value="每周" />
          <el-option label="每年" value="每年" />
        </el-select>
        <el-select v-model="newReminder.icon" style="width:80px">
          <el-option label="🏠" value="🏠" />
          <el-option label="💰" value="💰" />
          <el-option label="📱" value="📱" />
          <el-option label="💡" value="💡" />
          <el-option label="🔔" value="🔔" />
        </el-select>
        <el-button type="primary" @click="addReminder">添加</el-button>
      </div>
    </el-card>

    <!-- 数据管理 -->
    <el-card class="card-gap">
      <template #header><span>📦 数据管理</span></template>
      <div class="data-actions">
        <el-button type="primary" @click="exportCSV">📥 导出 CSV</el-button>
        <el-button type="success" @click="exportJSON">💾 备份数据 (JSON)</el-button>
        <el-upload :auto-upload="false" :show-file-list="false" accept=".json" @change="importJSON" style="display:inline-block;margin-left:10px">
          <el-button type="warning">📤 恢复备份</el-button>
        </el-upload>
      </div>
      <p class="hint-text">CSV 可用 Excel 打开；JSON 备份包含所有数据，可用于迁移</p>
    </el-card>

    <!-- 主题 -->
    <el-card class="card-gap">
      <template #header><span>🎨 外观</span></template>
      <el-switch v-model="store.isDark" @change="store.toggleTheme()" active-text="深色模式" inactive-text="浅色模式" />
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { useAppStore } from '../store'
import { getTransactions, getStatistics } from '../db'

const store = useAppStore()
const budget = ref(store.monthlyBudget)
const monthExpense = ref(0)

const newAccount = ref({ name: '', type: '储蓄卡' })
const newReminder = ref({ name: '', amount: 0, cycle: '每月', icon: '🏠' })

const budgetPercent = ref(0)

onMounted(async () => { await loadMonthExpense() })

async function loadMonthExpense() {
  const now = new Date()
  const start = `${now.getFullYear()}-${String(now.getMonth()+1).padStart(2,'0')}-01`
  const end = `${now.getFullYear()}-${String(now.getMonth()+1).padStart(2,'0')}-${String(new Date(now.getFullYear(),now.getMonth()+1,0).getDate()).padStart(2,'0')}`
  try {
    const stats = await getStatistics(start, end)
    monthExpense.value = stats.summary.total_expense
    if (budget.value > 0) budgetPercent.value = Math.round(monthExpense.value / budget.value * 100)
  } catch (e) { /* ignore */ }
}

function saveBudget() {
  store.setBudget(budget.value)
  if (budget.value > 0 && monthExpense.value > 0) {
    budgetPercent.value = Math.round(monthExpense.value / budget.value * 100)
  }
  ElMessage.success('预算已保存')
}

function addAccount() {
  if (!newAccount.value.name) { ElMessage.warning('请输入账户名'); return }
  store.addAccount({ ...newAccount.value })
  newAccount.value = { name: '', type: '储蓄卡' }
  ElMessage.success('账户已添加')
}

function addReminder() {
  if (!newReminder.value.name) { ElMessage.warning('请输入名称'); return }
  store.addReminder({ ...newReminder.value })
  newReminder.value = { name: '', amount: 0, cycle: '每月', icon: '🏠' }
  ElMessage.success('提醒已添加')
}

async function exportCSV() {
  const data = await getTransactions({})
  if (data.length === 0) { ElMessage.warning('没有数据可导出'); return }
  let csv = '﻿类型,金额,一级分类,二级分类,日期,备注\n'
  data.forEach(t => {
    csv += `${t.type === 'expense' ? '支出' : '收入'},${t.amount},${t.category_l1},${t.category_l2},${t.date},${t.note || ''}\n`
  })
  downloadFile(csv, `记账数据_${new Date().toISOString().slice(0,10)}.csv`, 'text/csv')
  ElMessage.success('CSV 已导出')
}

async function exportJSON() {
  const data = await getTransactions({})
  const json = JSON.stringify({ exportDate: new Date().toISOString(), transactions: data, accounts: store.accounts, reminders: store.reminders, budget: store.monthlyBudget }, null, 2)
  downloadFile(json, `记账备份_${new Date().toISOString().slice(0,10)}.json`, 'application/json')
  ElMessage.success('备份已导出')
}

async function importJSON(file) {
  try {
    const text = await file.raw.text()
    const json = JSON.parse(text)
    if (!json.transactions || !Array.isArray(json.transactions)) { ElMessage.error('无效的备份文件'); return }
    // 导入数据到 IndexedDB
    const { addTransaction } = await import('../db/browser.js')
    let count = 0
    for (const t of json.transactions) {
      await addTransaction({ type: t.type, amount: t.amount, category_l1: t.category_l1, category_l2: t.category_l2, date: t.date, note: t.note || '' })
      count++
    }
    if (json.accounts) { store.accounts = json.accounts; store.saveAccounts() }
    if (json.reminders) { store.reminders = json.reminders; store.saveReminders() }
    if (json.budget) store.setBudget(json.budget)
    ElMessage.success(`已导入 ${count} 条记录`)
  } catch (e) {
    ElMessage.error('导入失败：文件格式不正确')
  }
}

function downloadFile(content, filename, mime) {
  const blob = new Blob([content], { type: mime })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = filename; a.click()
  URL.revokeObjectURL(url)
}
</script>

<style scoped>
.settings { max-width: 700px; margin: 0 auto; }
.budget-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.budget-status { margin-top: 12px; }
.budget-text { font-size: 13px; color: var(--text-secondary); margin-left: 8px; }
.budget-warn { color: var(--red); font-weight: 600; margin-left: 8px; }
.add-row { display: flex; align-items: center; gap: 8px; margin-top: 12px; flex-wrap: wrap; }
.empty-hint { color: var(--text-muted); font-size: 13px; padding: 12px 0; }
.reminder-list { display: flex; flex-direction: column; gap: 8px; }
.reminder-item { display: flex; align-items: center; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid var(--border-hairline); }
.reminder-info { color: var(--text-secondary); font-size: 13px; }
.data-actions { display: flex; gap: 10px; flex-wrap: wrap; align-items: center; }
.hint-text { color: var(--text-muted); font-size: 12px; margin-top: 8px; }
</style>
