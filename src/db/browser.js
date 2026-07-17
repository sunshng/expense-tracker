// 浏览器端数据库 - 使用 IndexedDB (通过 Dexie.js)
import Dexie from 'dexie'

// 创建数据库
const db = new Dexie('ExpenseTracker')

// 定义表结构（v2：清除旧分类，避免重复）
db.version(2).stores({
  transactions: '++id, type, category_l1, category_l2, date, amount',
  categories: '++id, parent_id, name, sort_order'
}).upgrade(async tx => {
  // 清除旧分类数据（修复重复问题）
  await tx.table('categories').clear()
})

// ==================== 分类操作 ====================

// 初始化默认分类（避免重复：支出12类 + 收入1类）
const EXPENSE_CATEGORIES = [
  { name: '餐饮', icon: '🍽️', children: ['早餐', '午餐', '晚餐', '零食饮料', '聚餐请客', '外卖'] },
  { name: '交通', icon: '🚗', children: ['公交地铁', '出租车/网约车', '高铁/火车', '飞机', '加油', '停车费'] },
  { name: '购物', icon: '🛒', children: ['日用品', '数码产品', '家居用品', '宠物用品', '办公用品'] },
  { name: '住房', icon: '🏠', children: ['房租', '水电费', '燃气费', '物业费', '维修', '网费'] },
  { name: '娱乐', icon: '🎮', children: ['电影', '游戏', '旅游', '运动健身', 'KTV/酒吧', '书籍'] },
  { name: '医疗', icon: '🏥', children: ['门诊', '药品', '体检', '住院', '牙科'] },
  { name: '教育', icon: '📚', children: ['培训课程', '书本教材', '考试报名', '文具'] },
  { name: '通讯', icon: '📱', children: ['话费', '快递'] },
  { name: '服饰', icon: '👗', children: ['衣服', '鞋子', '包包', '饰品', '化妆品'] },
  { name: '亲子', icon: '👶', children: ['奶粉', '尿布', '玩具', '教育'] },
  { name: '人情', icon: '🎁', children: ['送礼', '红包', '捐款', '婚礼份子'] },
  { name: '其他支出', icon: '🔧', children: ['不确定分类'] },
]

const INCOME_CATEGORIES = [
  { name: '收入', icon: '💰', children: [
    '工资薪金', '年终奖金', '绩效提成', '兼职收入', '自由职业',
    '经营收入', '投资收益', '理财收益', '租金收入', '生活补贴',
    '红包收入', '礼金收入', '报销退款', '出售闲置', '其他收入'
  ]}
]

export async function getCategories() {
  let cats = await db.categories.orderBy('sort_order').toArray()

  // 如果表为空，插入默认分类
  if (cats.length === 0) {
    const allCategories = [...EXPENSE_CATEGORIES, ...INCOME_CATEGORIES]
    for (let i = 0; i < allCategories.length; i++) {
      const cat = allCategories[i]
      const parentId = await db.categories.put({
        name: cat.name,
        parent_id: null,
        icon: cat.icon,
        sort_order: i
      })
      for (let j = 0; j < cat.children.length; j++) {
        await db.categories.put({
          name: cat.children[j],
          parent_id: parentId,
          icon: '',
          sort_order: j
        })
      }
    }
    cats = await db.categories.orderBy('sort_order').toArray()
  }
  return cats
}

// ==================== 账单操作 ====================

export async function getTransactions(filters = {}) {
  let collection = db.transactions.orderBy('date').reverse()

  const items = await collection.toArray()

  // 在 JS 中筛选（Dexie 的筛选能力有限）
  let result = items

  if (filters.type) {
    result = result.filter(t => t.type === filters.type)
  }
  if (filters.category_l1) {
    result = result.filter(t => t.category_l1 === filters.category_l1)
  }
  if (filters.startDate) {
    result = result.filter(t => t.date >= filters.startDate)
  }
  if (filters.endDate) {
    result = result.filter(t => t.date <= filters.endDate)
  }
  if (filters.keyword) {
    result = result.filter(t => t.note && t.note.includes(filters.keyword))
  }

  // 按日期和ID倒序
  result.sort((a, b) => {
    if (b.date !== a.date) return b.date.localeCompare(a.date)
    return b.id - a.id
  })

  return result
}

export async function addTransaction(transaction) {
  const { type, amount, category_l1, category_l2, date, note } = transaction
  const id = await db.transactions.put({
    type,
    amount,
    category_l1,
    category_l2,
    date,
    note: note || '',
    created_at: new Date().toISOString()
  })
  return { id, ...transaction }
}

export async function deleteTransaction(id) {
  await db.transactions.delete(id)
  return { success: true }
}

export async function updateTransaction(id, transaction) {
  const { type, amount, category_l1, category_l2, date, note } = transaction
  await db.transactions.update(id, {
    type,
    amount,
    category_l1,
    category_l2,
    date,
    note: note || ''
  })
  return { success: true }
}

export async function getStatistics(startDate, endDate) {
  const items = await db.transactions.toArray()

  // 按日期筛选
  let filtered = items
  if (startDate) {
    filtered = filtered.filter(t => t.date >= startDate)
  }
  if (endDate) {
    filtered = filtered.filter(t => t.date <= endDate)
  }

  // 按分类统计支出
  const catMap = {}
  filtered.filter(t => t.type === 'expense').forEach(t => {
    if (!catMap[t.category_l1]) catMap[t.category_l1] = { total: 0, count: 0 }
    catMap[t.category_l1].total += t.amount
    catMap[t.category_l1].count++
  })
  const byCategory = Object.entries(catMap)
    .map(([category_l1, v]) => ({ category_l1, total: v.total, count: v.count }))
    .sort((a, b) => b.total - a.total)

  // 按月统计
  const monthMap = {}
  filtered.forEach(t => {
    const month = t.date.substring(0, 7)
    if (!monthMap[month]) monthMap[month] = { total_expense: 0, total_income: 0 }
    if (t.type === 'expense') monthMap[month].total_expense += t.amount
    else monthMap[month].total_income += t.amount
  })
  const monthly = Object.entries(monthMap)
    .map(([month, v]) => ({ month, ...v }))
    .sort((a, b) => a.month.localeCompare(b.month))
    .slice(-12)

  // 总收支
  const summary = {
    total_expense: filtered.filter(t => t.type === 'expense').reduce((s, t) => s + t.amount, 0),
    total_income: filtered.filter(t => t.type === 'income').reduce((s, t) => s + t.amount, 0)
  }

  return { byCategory, monthly, summary }
}
