<template>
  <div id="app-container">
    <!-- 桌面端：侧边导航栏 -->
    <el-container class="main-container desktop-layout">
      <el-aside width="200px" class="sidebar">
        <div class="logo">
          <span class="logo-icon">💰</span>
          <span class="logo-text">记账APP</span>
        </div>
        <el-menu
          :default-active="activeMenu"
          router
          class="sidebar-menu"
        >
          <el-menu-item index="/">
            <el-icon><List /></el-icon>
            <span>账单列表</span>
          </el-menu-item>
          <el-menu-item index="/add">
            <el-icon><Plus /></el-icon>
            <span>记一笔</span>
          </el-menu-item>
          <el-menu-item index="/statistics">
            <el-icon><DataAnalysis /></el-icon>
            <span>统计分析</span>
          </el-menu-item>
        </el-menu>
      </el-aside>

      <el-main class="main-content">
        <router-view />
      </el-main>
    </el-container>

    <!-- 移动端：底部导航栏 -->
    <div class="mobile-layout">
      <div class="mobile-content">
        <router-view />
      </div>
      <div class="bottom-nav">
        <div
          class="nav-item"
          :class="{ active: activeMenu === '/' }"
          @click="$router.push('/')"
        >
          <span class="nav-icon">📋</span>
          <span class="nav-label">账单</span>
        </div>
        <div
          class="nav-item nav-item-add"
          :class="{ active: activeMenu === '/add' }"
          @click="$router.push('/add')"
        >
          <div class="nav-add-btn">
            <span class="nav-add-icon">+</span>
          </div>
          <span class="nav-label">记一笔</span>
        </div>
        <div
          class="nav-item"
          :class="{ active: activeMenu === '/statistics' }"
          @click="$router.push('/statistics')"
        >
          <span class="nav-icon">📊</span>
          <span class="nav-label">统计</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { List, Plus, DataAnalysis } from '@element-plus/icons-vue'

const route = useRoute()
const activeMenu = computed(() => route.path)
</script>

<style scoped>
#app-container {
  height: 100vh;
  height: 100dvh;
  background: #f5f7fa;
}

/* ==================== 桌面布局 ==================== */
.desktop-layout {
  height: 100%;
}

.main-container {
  height: 100%;
}

.sidebar {
  background: #ffffff;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
}

.logo {
  padding: 20px 16px;
  display: flex;
  align-items: center;
  gap: 8px;
  border-bottom: 1px solid #f0f0f0;
}

.logo-icon {
  font-size: 24px;
}

.logo-text {
  font-size: 18px;
  font-weight: 700;
  color: #303133;
}

.sidebar-menu {
  border-right: none;
  margin-top: 8px;
}

.main-content {
  padding: 24px;
  overflow-y: auto;
}

/* ==================== 移动布局 ==================== */
.mobile-layout {
  display: none;
  height: 100vh;
  height: 100dvh;
  flex-direction: column;
}

.mobile-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  padding-bottom: 80px;
}

.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-around;
  align-items: center;
  background: #ffffff;
  border-top: 1px solid #e4e7ed;
  padding: 8px 0;
  padding-bottom: max(8px, env(safe-area-inset-bottom));
  height: 64px;
  z-index: 100;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  cursor: pointer;
  color: #909399;
  transition: color 0.2s;
  min-width: 60px;
}

.nav-item.active {
  color: #409eff;
}

.nav-item-add {
  position: relative;
  top: -20px;
}

.nav-add-btn {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: linear-gradient(135deg, #409eff, #66b1ff);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.4);
  margin-bottom: 2px;
}

.nav-add-icon {
  font-size: 28px;
  color: #ffffff;
  font-weight: 300;
  line-height: 1;
}

.nav-icon {
  font-size: 22px;
}

.nav-label {
  font-size: 11px;
}

/* ==================== 响应式切换 ==================== */
@media (max-width: 768px) {
  .desktop-layout {
    display: none !important;
  }
  .mobile-layout {
    display: flex;
  }
}

@media (min-width: 769px) {
  .desktop-layout {
    display: flex;
  }
  .mobile-layout {
    display: none;
  }
}
</style>

<!-- 全局样式覆盖 -->
<style>
/* 移动端全局调整 */
@media (max-width: 768px) {
  .el-message-box {
    width: 90% !important;
    max-width: 380px !important;
  }

  .el-dialog {
    width: 92% !important;
    max-width: 420px !important;
  }

  .el-select,
  .el-date-picker {
    width: 100% !important;
  }

  /* 移动端卡片间距 */
  .summary-row {
    flex-direction: column;
    gap: 10px !important;
  }

  .charts-row {
    flex-direction: column;
    gap: 12px !important;
  }

  .filter-bar {
    flex-wrap: wrap;
    gap: 8px !important;
  }

  .stat-card,
  .chart-card {
    width: 100% !important;
  }

  .chart-container {
    height: 250px !important;
  }

  .category-l1 .el-radio-group {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .category-l2 .el-radio-group {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .el-radio-button__inner {
    padding: 6px 10px !important;
    font-size: 12px !important;
  }

  .page-title {
    font-size: 18px !important;
    margin-bottom: 12px !important;
  }
}
</style>
