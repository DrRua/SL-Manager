<script setup lang="ts">
import { NCard, NButton, NInput, NForm, NFormItem, NSpace } from "naive-ui";
</script>

<template>
  <div class="app-container">
    <div class="cards-container">
      <NCard title="左侧卡片" bordered>
        <div class="card-content">
          <p>这是左侧卡片的内容区域，可以放置任何组件或文本。</p>
          <NSpace vertical>
            <NButton type="primary">左侧按钮 1</NButton>
            <NButton>左侧按钮 2</NButton>
          </NSpace>
        </div>
      </NCard>

      <NCard title="右侧卡片" bordered>
        <div class="card-content">
          <p>这是右侧卡片的内容区域，可以放置任何组件或文本。</p>
          <NForm>
            <NFormItem label="输入框">
              <NInput placeholder="请输入内容" />
            </NFormItem>
            <NButton type="primary">右侧按钮</NButton>
          </NForm>
        </div>
      </NCard>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: stretch;
  padding: 20px;
}

.cards-container {
  display: flex;
  gap: 20px;
  width: 100%;
  max-width: 1000px;
  height: 100%;
}

.cards-container :deep(.n-card) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.cards-container :deep(.n-card__body) {
  flex: 1;
  overflow: auto;
}

.card-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: 100%;
}

@media (max-width: 768px) {
  .cards-container {
    flex-direction: column;
  }
}

</style>