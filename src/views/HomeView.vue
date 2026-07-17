<template>
  <div class="home-view">
    <div class="page-header">
      <h2 class="page-title">账单列表</h2>
      <div class="header-actions">
        <el-button type="primary" @click="$router.push('/add')">
          <el-icon><Plus /></el-icon>
          记一笔
        </el-button>
      </div>
    </div>

    <!-- 月度汇总卡片 -->
    <div class="summary-row" v-if="summary">
      <el-card class="summary-card expense-card">
        <div class="summary-label">本月支出</div>
        <div class="summary-amount expense">¥{{ formatAmount(summary.total_expense) }}</div>
      </el-card>
      <el-card class="summary-card income-card">
        <div class="summary-label">本月收入</div>
        <div class="summary-amount income">¥{{ formatAmount(summary.total_income) }}</div>
      </el-card>
      <el-card class="summary-card balance-card">
        <div class="summary-label">本月结余</div>
        <div class="summary-amount" :class="balanceClass">
          ¥{{ formatAmount(summary.total_income - summary.total_expense) }}
        </div>
      </el-card>
    </div>

    <!-- 筛选栏 -->
    <div class="filter-bar">
      <el-select v-model="filters.type" placeholder="全部类型" clearable style="width:120px">
        <el-option label="支出" value="expense" />
        <el-option label="收入" value="income" />
      </el-select>
      <el-select v-model="filters.category_l1" placeholder="全部分类" clearable style="width:140px">
        <el-option
          v-for="cat in store.categories.filter(c => !c.parent_id)"
          :key="cat.id"
          :label="cat.icon + ' ' + cat.name"
          :value="cat.name"
        />
      </el-select>
      <el-date-picker
        v-model="filters.dateRange"
        type="month"
        placeholder="选择月份"
        value-format="YYYY-MM"
        style="width:160px"
      />
      <el-input v-model="filters.keyword" placeholder="🔍 搜索备注..." clearable style="width:160px" @clear="loadTransactions" @keyup.enter="loadTransactions" />
      <el-button @click="loadTransactions" type="primary" plain>筛选</el-button>
      <el-button @click="resetFilters" plain>重置</el-button>
      <el-button @click="exportCurrentCSV" plain>📥 导出</el-button>
    </div>

    <!-- 账单列表 -->
    <el-card class="list-card" v-loading="loading">
      <div v-if="transactions.length === 0 && !loading" class="empty-state">
        <div class="empty-icon">📋</div>
        <div class="empty-text">暂无账单记录</div>
        <el-button type="primary" @click="$router.push('/add')">记第一笔</el-button>
      </div>

      <div v-else class="transaction-list">
        <div
          v-for="(group, date) in groupedTransactions"
          :key="date"
          class="date-group"
        >
          <!-- 日期标题 -->
          <div class="date-header">
            <span class="date-text">{{ formatDate(date) }}</span>
            <span class="date-summary">
              {{ getDateSummary(group) }}
            </span>
          </div>

          <!-- 该日期下的账单条目 -->
          <div
            v-for="item in group"
            :key="item.id"
            class="transaction-item"
          >
            <div class="item-left">
              <span class="item-icon">{{ getCategoryIcon(item.category_l1) }}</span>
              <div class="item-info">
                <div class="item-category">{{ item.category_l1 }} · {{ item.category_l2 }}</div>
                <div class="item-note" v-if="item.note">{{ item.note }}</div>
              </div>
            </div>
            <div class="item-right">
              <span
                class="item-amount"
                :class="item.type === 'income' ? 'income' : 'expense'"
              >
                {{ item.type === 'income' ? '+' : '-' }}¥{{ formatAmount(item.amount) }}
              </span>
              <el-dropdown trigger="click" @command="(cmd) => handleAction(cmd, item)">
                <el-button text circle>
                  <el-icon><MoreFilled /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item command="edit">编辑</el-dropdown-item>
                    <el-dropdown-item command="delete" divided style="color:#f56c6c">
                      删除
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 编辑对话框 -->
    <el-dialog v-model="editDialogVisible" title="编辑账单" width="450px">
      <el-form :model="editForm" label-position="top" v-if="editForm">
        <el-form-item label="金额（元）">
          <el-input v-model="editForm.amount" type="number" />
        </el-form-item>
        <el-form-item label="日期">
          <el-date-picker
            v-model="editForm.date"
            type="date"
            value-format="YYYY-MM-DD"
            style="width:100%"
          />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="editForm.note" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveEdit">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, MoreFilled } from '@element-plus/icons-vue'
import { getTransactions, deleteTransaction, updateTransaction, getStatistics } from '../db'
import { useAppStore } from '../store'

const store = useAppStore()
const transactions = ref([])
const loading = ref(false)
const summary = ref(null)

// 筛选条件
const filters = reactive({
  type: '',
  category_l1: '',
  dateRange: '',
  keyword: ''
})

// 编辑相关
const editDialogVisible = ref(false)
const editForm = ref(null)
const editId = ref(null)

// 按日期分组
const groupedTransactions = computed(() => {
  const groups = {}
  transactions.value.forEach(item => {
    if (!groups[item.date]) {
      groups[item.date] = []
    }
    groups[item.date].push(item)
  })
  return groups
})

// 结余样式
const balanceClass = computed(() => {
  if (!summary.value) return ''
  const balance = summary.value.total_income - summary.value.total_expense
  return balance >= 0 ? 'income' : 'expense'
})

// 加载账单
async function loadTransactions() {
  loading.value = true
  try {
    const filterParams = {}

    if (filters.type) {
      filterParams.type = filters.type
    }
    if (filters.category_l1) {
      filterParams.category_l1 = filters.category_l1
    }
    if (filters.dateRange) {
      const [year, month] = filters.dateRange.split('-')
      filterParams.startDate = `${year}-${month}-01`
      // 计算月末日期
      const lastDay = new Date(parseInt(year), parseInt(month), 0).getDate()
      filterParams.endDate = `${year}-${month}-${String(lastDay).padStart(2, '0')}`
    }

    transactions.value = await getTransactions(filterParams)

    // 获取月度汇总
    const now = new Date()
    const startOfMonth = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-01`
    const endOfMonth = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(new Date(now.getFullYear(), now.getMonth() + 1, 0).getDate()).padStart(2, '0')}`

    const statsResult = await getStatistics(
      filters.dateRange ? filterParams.startDate : startOfMonth,
      filters.dateRange ? filterParams.endDate : endOfMonth
    )
    summary.value = statsResult.summary
  } catch (error) {
    ElMessage.error('加载账单失败')
    console.error(error)
  } finally {
    loading.value = false
  }
}

// 重置筛选
function resetFilters() {
  filters.type = ''
  filters.category_l1 = ''
  filters.dateRange = ''
  filters.keyword = ''
  loadTransactions()
}

// 导出当前筛选结果为 CSV
function exportCurrentCSV() {
  let csv = '﻿类型,金额,一级分类,二级分类,日期,备注\n'
  transactions.value.forEach(t => {
    csv += `${t.type === 'expense' ? '支出' : '收入'},${t.amount},${t.category_l1},${t.category_l2},${t.date},${t.note || ''}\n`
  })
  if (transactions.value.length === 0) { ElMessage.warning('没有数据可导出'); return }
  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = `记账数据_${new Date().toISOString().slice(0,10)}.csv`; a.click()
  URL.revokeObjectURL(url)
  ElMessage.success('CSV 已导出')
}

// 格式化金额
function formatAmount(amount) {
  if (amount == null) return '0.00'
  return Number(amount).toFixed(2)
}

// 格式化日期显示
function formatDate(dateStr) {
  const date = new Date(dateStr)
  const weekDays = ['周日', '周一', '周二', '周三', '周四', '周五', '周六']
  const month = date.getMonth() + 1
  const day = date.getDate()
  const weekDay = weekDays[date.getDay()]
  return `${month}月${day}日 ${weekDay}`
}

// 获取日期组的汇总
function getDateSummary(group) {
  let expenseTotal = 0
  let incomeTotal = 0
  group.forEach(item => {
    if (item.type === 'expense') expenseTotal += item.amount
    else incomeTotal += item.amount
  })
  let summary = ''
  if (expenseTotal > 0) summary += `支出 ¥${expenseTotal.toFixed(2)}`
  if (incomeTotal > 0) {
    if (summary) summary += '  '
    summary += `收入 ¥${incomeTotal.toFixed(2)}`
  }
  return summary
}

// 获取分类图标
function getCategoryIcon(categoryName) {
  const cat = store.categories.find(c => c.name === categoryName)
  return cat ? cat.icon : '📌'
}

// 操作处理
function handleAction(command, item) {
  if (command === 'delete') {
    handleDelete(item)
  } else if (command === 'edit') {
    handleEdit(item)
  }
}

// 编辑账单
function handleEdit(item) {
  editId.value = item.id
  editForm.value = {
    amount: item.amount,
    date: item.date,
    note: item.note
  }
  editDialogVisible.value = true
}

// 保存编辑
async function saveEdit() {
  try {
    await updateTransaction(editId.value, {
      type: transactions.value.find(t => t.id === editId.value).type,
      amount: parseFloat(editForm.value.amount),
      category_l1: transactions.value.find(t => t.id === editId.value).category_l1,
      category_l2: transactions.value.find(t => t.id === editId.value).category_l2,
      date: editForm.value.date,
      note: editForm.value.note
    })
    ElMessage.success('账单已更新')
    editDialogVisible.value = false
    loadTransactions()
  } catch (error) {
    ElMessage.error('更新失败')
    console.error(error)
  }
}

// 删除账单
async function handleDelete(item) {
  try {
    await ElMessageBox.confirm(
      `确定要删除「${item.category_l1}·${item.category_l2}」¥${item.amount.toFixed(2)} 这条记录吗？`,
      '确认删除',
      { confirmButtonText: '删除', cancelButtonText: '取消', type: 'warning' }
    )
    await deleteTransaction(item.id)
    ElMessage.success('已删除')
    loadTransactions()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
      console.error(error)
    }
  }
}

onMounted(async () => {
  await store.loadCategories()
  await loadTransactions()
})
</script>

<style scoped>
.home-view {
  max-width: 900px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.page-title {
  margin: 0;
  font-size: 22px;
  color: #303133;
}

.summary-row {
  display: flex;
  gap: 16px;
  margin-bottom: 20px;
}

.summary-card {
  flex: 1;
  text-align: center;
  padding: 8px 0;
}

.summary-label {
  font-size: 13px;
  color: #909399;
  margin-bottom: 6px;
}

.summary-amount {
  font-size: 22px;
  font-weight: 700;
}

.summary-amount.expense {
  color: #f56c6c;
}

.summary-amount.income {
  color: #67c23a;
}

.filter-bar {
  display: flex;
  gap: 10px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}

.list-card {
  min-height: 300px;
}

.empty-state {
  text-align: center;
  padding: 60px 0;
  color: #909399;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-text {
  font-size: 15px;
  margin-bottom: 20px;
}

.date-group {
  margin-bottom: 16px;
}

.date-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
  margin-bottom: 4px;
}

.date-text {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.date-summary {
  font-size: 12px;
  color: #909399;
}

.transaction-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 8px;
  border-bottom: 1px solid #fafafa;
  transition: background 0.2s;
}

.transaction-item:hover {
  background: #f5f7fa;
}

.item-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.item-icon {
  font-size: 28px;
  width: 40px;
  text-align: center;
}

.item-category {
  font-size: 14px;
  color: #303133;
}

.item-note {
  font-size: 12px;
  color: #c0c4cc;
  margin-top: 2px;
}

.item-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.item-amount {
  font-size: 16px;
  font-weight: 600;
  font-family: 'Courier New', monospace;
}

.item-amount.expense {
  color: #f56c6c;
}

.item-amount.income {
  color: #67c23a;
}
</style>
