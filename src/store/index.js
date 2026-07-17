import { defineStore } from 'pinia'
import { getCategories } from '../db'

// 全局状态管理
export const useAppStore = defineStore('app', {
  state: () => ({
    // 分类数据
    categories: [],
    // 当前选中的分类
    currentCategoryL1: '',
    currentCategoryL2: ''
  }),

  getters: {
    // 获取支出类的一级分类（排除收入）
    expenseCategories: (state) => {
      return state.categories.filter(cat => cat.parent_id === null && cat.name !== '收入')
    },
    // 获取收入类的一级分类
    incomeCategories: (state) => {
      return state.categories.filter(cat => cat.parent_id === null && cat.name === '收入')
    },
    // 根据一级分类名称获取二级分类列表
    subCategories: (state) => (parentName) => {
      const parent = state.categories.find(cat => cat.name === parentName && cat.parent_id === null)
      if (!parent) return []
      return state.categories.filter(cat => cat.parent_id === parent.id)
    }
  },

  actions: {
    // 加载分类数据
    async loadCategories() {
      try {
        this.categories = await getCategories()
      } catch (error) {
        console.error('加载分类失败:', error)
      }
    }
  }
})
