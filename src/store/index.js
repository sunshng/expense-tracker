import { defineStore } from 'pinia'
import { getCategories } from '../db'

// 全局状态管理
export const useAppStore = defineStore('app', {
  state: () => ({
    // 分类数据
    categories: [],
    // 当前选中的分类
    currentCategoryL1: '',
    currentCategoryL2: '',
    // 主题：'light' | 'dark'
    theme: localStorage.getItem('app-theme') || 'light',
    // 月度预算（元），0 表示未设置
    monthlyBudget: parseFloat(localStorage.getItem('monthly-budget') || '0'),
    // 账户列表
    accounts: JSON.parse(localStorage.getItem('accounts') || '[]'),
    // 周期提醒
    reminders: JSON.parse(localStorage.getItem('reminders') || '[]')
  }),

  getters: {
    expenseCategories: (state) => {
      return state.categories.filter(cat => cat.parent_id === null && cat.name !== '收入')
    },
    incomeCategories: (state) => {
      return state.categories.filter(cat => cat.parent_id === null && cat.name === '收入')
    },
    subCategories: (state) => (parentName) => {
      const parent = state.categories.find(cat => cat.name === parentName && cat.parent_id === null)
      if (!parent) return []
      return state.categories.filter(cat => cat.parent_id === parent.id)
    },
    isDark: (state) => state.theme === 'dark'
  },

  actions: {
    async loadCategories() {
      try {
        this.categories = await getCategories()
      } catch (error) {
        console.error('加载分类失败:', error)
      }
    },

    // 切换主题
    toggleTheme() {
      this.theme = this.theme === 'light' ? 'dark' : 'light'
      localStorage.setItem('app-theme', this.theme)
      document.documentElement.setAttribute('data-theme', this.theme)
    },

    // 应用主题（初始化时调用）
    applyTheme() {
      document.documentElement.setAttribute('data-theme', this.theme)
    },

    // 设置月度预算
    setBudget(amount) {
      this.monthlyBudget = amount
      localStorage.setItem('monthly-budget', String(amount))
    },

    // 账户管理
    addAccount(account) {
      this.accounts.push({ id: Date.now(), ...account })
      this.saveAccounts()
    },
    deleteAccount(id) {
      this.accounts = this.accounts.filter(a => a.id !== id)
      this.saveAccounts()
    },
    saveAccounts() {
      localStorage.setItem('accounts', JSON.stringify(this.accounts))
    },

    // 周期提醒
    addReminder(reminder) {
      this.reminders.push({ id: Date.now(), ...reminder })
      this.saveReminders()
    },
    deleteReminder(id) {
      this.reminders = this.reminders.filter(r => r.id !== id)
      this.saveReminders()
    },
    saveReminders() {
      localStorage.setItem('reminders', JSON.stringify(this.reminders))
    }
  }
})
