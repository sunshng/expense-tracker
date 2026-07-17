<template>
  <div class="add-record">
    <h2 class="page-title">记一笔</h2>

    <el-card class="form-card">
      <!-- 收支类型切换 -->
      <div class="type-switch">
        <el-radio-group v-model="form.type" size="large">
          <el-radio-button value="expense">
            <span class="type-label">💸 支出</span>
          </el-radio-button>
          <el-radio-button value="income">
            <span class="type-label">💰 收入</span>
          </el-radio-button>
        </el-radio-group>
      </div>

      <!-- 金额输入 -->
      <div class="amount-input">
        <span class="currency-symbol">¥</span>
        <el-input
          v-model="form.amount"
          placeholder="0.00"
          class="amount-field"
          @input="validateAmount"
        />
      </div>

      <!-- 分类选择 -->
      <div class="category-section">
        <div class="section-label">选择分类</div>

        <!-- 一级分类 -->
        <div class="category-l1">
          <el-radio-group v-model="form.category_l1">
            <el-radio-button
              v-for="cat in availableL1Categories"
              :key="cat.id"
              :value="cat.name"
            >
              {{ cat.icon }} {{ cat.name }}
            </el-radio-button>
          </el-radio-group>
        </div>

        <!-- 二级分类 -->
        <div class="category-l2" v-if="form.category_l1">
          <div class="section-label">具体分类</div>
          <el-radio-group v-model="form.category_l2">
            <el-radio-button
              v-for="cat in availableL2Categories"
              :key="cat.id"
              :value="cat.name"
            >
              {{ cat.name }}
            </el-radio-button>
          </el-radio-group>
        </div>
      </div>

      <!-- 日期选择 -->
      <div class="form-row">
        <div class="section-label">日期</div>
        <el-date-picker
          v-model="form.date"
          type="date"
          placeholder="选择日期"
          value-format="YYYY-MM-DD"
          class="date-picker"
        />
      </div>

      <!-- 备注 -->
      <div class="form-row">
        <div class="section-label">备注（可选）</div>
        <el-input
          v-model="form.note"
          placeholder="例如：和同事聚餐"
          maxlength="100"
          show-word-limit
        />
      </div>

      <!-- 提交按钮 -->
      <div class="form-actions">
        <el-button
          type="primary"
          size="large"
          @click="submitRecord"
          :disabled="!canSubmit"
          :loading="submitting"
        >
          确认记录
        </el-button>
      </div>
    </el-card>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { addTransaction } from '../db'
import { useAppStore } from '../store'

const router = useRouter()
const store = useAppStore()

const submitting = ref(false)

// 表单数据
const form = reactive({
  type: 'expense',
  amount: '',
  category_l1: '',
  category_l2: '',
  date: new Date().toISOString().slice(0, 10),
  note: ''
})

// 可用的一级分类（根据收支类型切换）
const availableL1Categories = computed(() => {
  if (form.type === 'income') {
    return store.incomeCategories
  }
  return store.expenseCategories
})

// 可用的二级分类
const availableL2Categories = computed(() => {
  if (!form.category_l1) return []
  return store.subCategories(form.category_l1)
})

// 是否可以提交
const canSubmit = computed(() => {
  if (!form.amount || parseFloat(form.amount) <= 0) return false
  if (!form.category_l1) return false
  if (!form.category_l2) return false
  if (!form.date) return false
  if (!submitting.value) return true
  return false
})

// 金额校验：只允许数字和最多两位小数
function validateAmount(value) {
  // 移除非数字字符（保留小数点）
  let cleaned = value.replace(/[^\d.]/g, '')
  // 只保留第一个小数点
  const parts = cleaned.split('.')
  if (parts.length > 2) {
    cleaned = parts[0] + '.' + parts.slice(1).join('')
  }
  // 限制两位小数
  if (parts.length === 2 && parts[1].length > 2) {
    cleaned = parts[0] + '.' + parts[1].slice(0, 2)
  }
  form.amount = cleaned
}

// 提交记录
async function submitRecord() {
  if (!canSubmit.value) return

  submitting.value = true
  try {
    await addTransaction({
      type: form.type,
      amount: parseFloat(form.amount),
      category_l1: form.category_l1,
      category_l2: form.category_l2,
      date: form.date,
      note: form.note
    })

    const typeLabel = form.type === 'expense' ? '支出' : '收入'
    ElMessage.success(`${typeLabel} ¥${parseFloat(form.amount).toFixed(2)} 记录成功！`)

    // 重置表单
    form.amount = ''
    form.category_l1 = ''
    form.category_l2 = ''
    form.note = ''

    // 跳转到账单列表
    router.push('/')
  } catch (error) {
    ElMessage.error('记录失败，请重试')
    console.error('添加账单失败:', error)
  } finally {
    submitting.value = false
  }
}

onMounted(async () => {
  await store.loadCategories()
})
</script>

<style scoped>
.add-record {
  max-width: 700px;
  margin: 0 auto;
}

.page-title {
  margin: 0 0 20px 0;
  font-size: 22px;
  color: #303133;
}

.form-card {
  padding: 10px;
}

.type-switch {
  text-align: center;
  margin-bottom: 24px;
}

.type-label {
  font-size: 15px;
}

.amount-input {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 28px;
  gap: 8px;
}

.currency-symbol {
  font-size: 36px;
  font-weight: 700;
  color: #409eff;
}

.amount-field {
  width: 240px;
}

.amount-field :deep(.el-input__inner) {
  font-size: 36px;
  font-weight: 700;
  color: #303133;
  text-align: center;
  height: 60px;
  border: none;
  border-bottom: 2px solid #dcdfe6;
  border-radius: 0;
}

.amount-field :deep(.el-input__inner):focus {
  border-bottom-color: #409eff;
}

.category-section {
  margin-bottom: 20px;
}

.section-label {
  font-size: 14px;
  color: #909399;
  margin-bottom: 10px;
  font-weight: 500;
}

.category-l1 {
  margin-bottom: 16px;
}

.category-l1 :deep(.el-radio-button__inner) {
  min-width: 70px;
  font-size: 13px;
  padding: 8px 12px;
}

.category-l2 {
  margin-top: 12px;
}

.category-l2 :deep(.el-radio-button__inner) {
  font-size: 13px;
  padding: 6px 14px;
}

.form-row {
  margin-bottom: 20px;
}

.date-picker {
  width: 200px;
}

.form-actions {
  text-align: center;
  margin-top: 32px;
  padding-top: 20px;
  border-top: 1px solid #f0f0f0;
}

.form-actions .el-button {
  width: 200px;
}
</style>
