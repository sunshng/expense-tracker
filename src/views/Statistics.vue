<template>
  <div class="statistics">
    <h2 class="page-title">统计分析</h2>

    <!-- 时间筛选 -->
    <div class="filter-row">
      <el-radio-group v-model="period" @change="loadStats">
        <el-radio-button value="month">本月</el-radio-button>
        <el-radio-button value="quarter">近3个月</el-radio-button>
        <el-radio-button value="year">今年</el-radio-button>
      </el-radio-group>
    </div>

    <!-- 总览卡片 -->
    <div class="summary-row" v-if="stats">
      <el-card class="stat-card">
        <div class="stat-label">总支出</div>
        <div class="stat-value expense">¥{{ formatAmount(stats.summary?.total_expense) }}</div>
      </el-card>
      <el-card class="stat-card">
        <div class="stat-label">总收入</div>
        <div class="stat-value income">¥{{ formatAmount(stats.summary?.total_income) }}</div>
      </el-card>
      <el-card class="stat-card">
        <div class="stat-label">结余</div>
        <div class="stat-value" :class="balanceClass">
          ¥{{ formatAmount((stats.summary?.total_income || 0) - (stats.summary?.total_expense || 0)) }}
        </div>
      </el-card>
    </div>

    <!-- 图表区域 -->
    <div class="charts-row" v-if="stats">
      <!-- 分类饼图 -->
      <el-card class="chart-card">
        <template #header>
          <span>支出分类占比</span>
        </template>
        <div class="chart-container" ref="pieChartRef"></div>
      </el-card>

      <!-- 月度趋势图 -->
      <el-card class="chart-card">
        <template #header>
          <span>月度收支趋势</span>
        </template>
        <div class="chart-container" ref="trendChartRef"></div>
      </el-card>
    </div>

    <!-- 分类排行 -->
    <el-card class="rank-card" v-if="stats && stats.byCategory && stats.byCategory.length > 0">
      <template #header>
        <span>支出分类排行</span>
      </template>
      <div class="rank-list">
        <div
          v-for="(item, index) in stats.byCategory"
          :key="item.category_l1"
          class="rank-item"
        >
          <div class="rank-left">
            <span class="rank-index" :class="'rank-' + (index + 1)">{{ index + 1 }}</span>
            <span class="rank-icon">{{ getCategoryIcon(item.category_l1) }}</span>
            <span class="rank-name">{{ item.category_l1 }}</span>
          </div>
          <div class="rank-right">
            <div class="rank-bar-wrapper">
              <div
                class="rank-bar"
                :style="{ width: getBarWidth(item.total) + '%' }"
              ></div>
            </div>
            <span class="rank-amount">¥{{ formatAmount(item.total) }}</span>
            <span class="rank-count">{{ item.count }}笔</span>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 无数据提示 -->
    <div v-if="stats && (!stats.byCategory || stats.byCategory.length === 0)" class="empty-state">
      <div class="empty-icon">📊</div>
      <div class="empty-text">该时间段暂无数据</div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue'
import * as echarts from 'echarts'
import { getStatistics } from '../db'
import { useAppStore } from '../store'

const store = useAppStore()

const period = ref('month')
const stats = ref(null)
const pieChartRef = ref(null)
const trendChartRef = ref(null)
let pieChart = null
let trendChart = null
let resizeHandler = null

// 结余样式
const balanceClass = computed(() => {
  if (!stats.value?.summary) return ''
  const balance = (stats.value.summary.total_income || 0) - (stats.value.summary.total_expense || 0)
  return balance >= 0 ? 'income' : 'expense'
})

// 获取时间范围
function getDateRange() {
  const now = new Date()
  const year = now.getFullYear()
  const month = now.getMonth() + 1

  if (period.value === 'month') {
    const lastDay = new Date(year, month, 0).getDate()
    return {
      startDate: `${year}-${String(month).padStart(2, '0')}-01`,
      endDate: `${year}-${String(month).padStart(2, '0')}-${String(lastDay).padStart(2, '0')}`
    }
  } else if (period.value === 'quarter') {
    const startMonth = month - 2
    const startYear = startMonth <= 0 ? year - 1 : year
    const adjustedStartMonth = startMonth <= 0 ? startMonth + 12 : startMonth
    const lastDay = new Date(year, month, 0).getDate()
    return {
      startDate: `${startYear}-${String(adjustedStartMonth).padStart(2, '0')}-01`,
      endDate: `${year}-${String(month).padStart(2, '0')}-${String(lastDay).padStart(2, '0')}`
    }
  } else if (period.value === 'year') {
    return {
      startDate: `${year}-01-01`,
      endDate: `${year}-12-31`
    }
  }
}

// 加载统计数据
async function loadStats() {
  try {
    const dateRange = getDateRange()
    const result = await getStatistics(dateRange.startDate, dateRange.endDate)
    stats.value = result
    // 等 DOM 更新后再渲染图表
    setTimeout(() => {
      renderPieChart()
      renderTrendChart()
    }, 100)
  } catch (error) {
    console.error('加载统计数据失败:', error)
  }
}

// 渲染饼图
function renderPieChart() {
  if (!pieChartRef.value || !stats.value?.byCategory || stats.value.byCategory.length === 0) return

  if (!pieChart) {
    pieChart = echarts.init(pieChartRef.value)
  }

  const data = stats.value.byCategory.map(item => ({
    name: item.category_l1,
    value: item.total
  }))

  pieChart.setOption({
    tooltip: {
      trigger: 'item',
      formatter: '{b}: ¥{c} ({d}%)'
    },
    series: [
      {
        type: 'pie',
        radius: ['45%', '75%'],
        center: ['50%', '50%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 4,
          borderColor: '#fff',
          borderWidth: 2
        },
        label: {
          show: true,
          formatter: '{b}\n{d}%',
          fontSize: 11
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 14,
            fontWeight: 'bold'
          }
        },
        data: data
      }
    ]
  })
}

// 渲染趋势图
function renderTrendChart() {
  if (!trendChartRef.value || !stats.value?.monthly || stats.value.monthly.length === 0) return

  if (!trendChart) {
    trendChart = echarts.init(trendChartRef.value)
  }

  const months = stats.value.monthly.map(item => {
    const parts = item.month.split('-')
    return parts[1] + '月'
  })
  const expenses = stats.value.monthly.map(item => item.total_expense)
  const incomes = stats.value.monthly.map(item => item.total_income)

  trendChart.setOption({
    tooltip: {
      trigger: 'axis'
    },
    legend: {
      data: ['支出', '收入'],
      bottom: 0
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '40px',
      top: '10px',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      data: months,
      axisLine: { lineStyle: { color: '#e4e7ed' } },
      axisTick: { show: false }
    },
    yAxis: {
      type: 'value',
      axisLabel: {
        formatter: '¥{value}'
      },
      splitLine: { lineStyle: { color: '#f0f0f0', type: 'dashed' } }
    },
    series: [
      {
        name: '支出',
        type: 'bar',
        data: expenses,
        itemStyle: {
          color: '#f56c6c',
          borderRadius: [4, 4, 0, 0]
        }
      },
      {
        name: '收入',
        type: 'bar',
        data: incomes,
        itemStyle: {
          color: '#67c23a',
          borderRadius: [4, 4, 0, 0]
        }
      }
    ]
  })
}

// 窗口大小变化时重绘图表
function handleResize() {
  pieChart?.resize()
  trendChart?.resize()
}

function formatAmount(amount) {
  if (amount == null) return '0.00'
  return Number(amount).toFixed(2)
}

function getCategoryIcon(categoryName) {
  const cat = store.categories.find(c => c.name === categoryName)
  return cat ? cat.icon : '📌'
}

function getBarWidth(amount) {
  if (!stats.value?.byCategory || stats.value.byCategory.length === 0) return 0
  const maxAmount = stats.value.byCategory[0].total
  if (maxAmount === 0) return 0
  return Math.round((amount / maxAmount) * 100)
}

// 监听分类数据变化，分类加载完成后重新渲染图表
watch(() => store.categories.length, () => {
  if (stats.value) {
    renderPieChart()
  }
})

onMounted(async () => {
  await store.loadCategories()
  await loadStats()
  resizeHandler = handleResize
  window.addEventListener('resize', resizeHandler)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', resizeHandler)
  pieChart?.dispose()
  trendChart?.dispose()
})
</script>

<style scoped>
.statistics {
  max-width: 900px;
  margin: 0 auto;
}

.page-title {
  margin: 0 0 20px 0;
  font-size: 22px;
  color: #303133;
}

.filter-row {
  margin-bottom: 20px;
}

.summary-row {
  display: flex;
  gap: 16px;
  margin-bottom: 20px;
}

.stat-card {
  flex: 1;
  text-align: center;
  padding: 8px 0;
}

.stat-label {
  font-size: 13px;
  color: #909399;
  margin-bottom: 6px;
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
}

.stat-value.expense {
  color: #f56c6c;
}

.stat-value.income {
  color: #67c23a;
}

.charts-row {
  display: flex;
  gap: 16px;
  margin-bottom: 20px;
}

.chart-card {
  flex: 1;
}

.chart-container {
  width: 100%;
  height: 320px;
}

.rank-card {
  margin-bottom: 20px;
}

.rank-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.rank-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.rank-left {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 120px;
}

.rank-index {
  width: 22px;
  height: 22px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
  color: #fff;
  background: #909399;
}

.rank-index.rank-1 {
  background: #f56c6c;
}

.rank-index.rank-2 {
  background: #e6a23c;
}

.rank-index.rank-3 {
  background: #409eff;
}

.rank-name {
  font-size: 14px;
  color: #303133;
}

.rank-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  margin-left: 16px;
}

.rank-bar-wrapper {
  flex: 1;
  height: 8px;
  background: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
}

.rank-bar {
  height: 100%;
  background: linear-gradient(90deg, #409eff, #66b1ff);
  border-radius: 4px;
  transition: width 0.5s ease;
}

.rank-amount {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
  min-width: 80px;
  text-align: right;
  font-family: 'Courier New', monospace;
}

.rank-count {
  font-size: 12px;
  color: #c0c4cc;
  min-width: 40px;
  text-align: right;
}

.empty-state {
  text-align: center;
  padding: 80px 0;
  color: #909399;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-text {
  font-size: 15px;
}
</style>
